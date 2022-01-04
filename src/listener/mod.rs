use crate::client::ApiClient;
use crate::config::Configuration;
use anyhow::Result;
use chan::chan_select;
use chrono::{DateTime, TimeZone, Utc};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub async fn listen(
    cfg: Arc<Configuration>,
    client: Arc<ApiClient>,
    quit: chan::Receiver<()>,
) -> () {
    let listener = Listener::init(cfg, client.clone()).unwrap();
    listener.run(quit).await
}

type State = HashMap<String, DateTime<Utc>>;

struct Listener {
    cfg: Arc<Configuration>,
    client: Arc<ApiClient>,
    state: Mutex<State>,
}

impl Listener {
    fn init(cfg: Arc<Configuration>, client: Arc<ApiClient>) -> Result<Self> {
        let mut state = State::new();
        for doorbell in client.get_doorbells()? {
            info!("listening doorbell {} (id: {}, mac: {})",  doorbell.name, doorbell.id, doorbell.mac);
            state.insert(
                doorbell.id,
                Utc.timestamp_millis(doorbell.last_ring.unwrap_or(0)),
            );
        }
        Ok(Self {
            cfg,
            client,
            state: Mutex::new(state),
        })
    }

    fn poll(&self) -> Result<()> {
        self.client
            .get_doorbells()?
            .into_iter()
            .filter(|doorbell| {
                let state = self.state.lock().unwrap();
                match state.get(doorbell.id.as_str()) {
                    Some(last_ring) => {
                        doorbell.last_ring.unwrap_or(0) > last_ring.timestamp_millis()
                    }
                    None => false,
                }
            })
            .for_each(|doorbell| {
                info!("doorbell {} is ringing!", doorbell.id);
                let mut state = self.state.lock().unwrap();

                let last_ring = Utc.timestamp_millis(doorbell.last_ring.unwrap_or(0).clone());
                *state.entry(doorbell.id.clone()).or_insert(last_ring) = last_ring;
                let url = format!(
                    "http://localhost:{}/ringing/{}",
                    self.cfg.server.port, doorbell.id
                );

                if self.cfg.open_browser {
                    if webbrowser::open(url.as_str()).is_err() {
                        error!("failed to open browser");
                    }
                }
            });
        Ok(())
    }

    async fn run(&self, quit: chan::Receiver<()>) -> () {
        info!("waiting chime...");

        let tick = chan::tick(Duration::from_secs(1));
        loop {
            chan_select! {
                quit.recv() => {
                    break
                },
                tick.recv() => {
                    if let Err(err) = self.poll() {
                        error!("failed polling: {}", err);
                    }
                },
            }
        }
        info!("finished listening");
        ()
    }
}
