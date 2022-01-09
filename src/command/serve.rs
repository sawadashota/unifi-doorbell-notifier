use crate::client::ApiClient;
use crate::command::config_path;
use crate::config::Configuration;
use crate::listener::listen;
use crate::server::{api, frontend};
use actix_web::middleware::{Compress, Logger};
use actix_web::rt::{spawn, System};
use actix_web::{web, App, HttpServer};
use anyhow::Result;
use log::LevelFilter;
use std::path::PathBuf;
use std::sync::{mpsc, Arc};
use std::thread;
use structopt::StructOpt;

/// Serve application
#[derive(StructOpt)]
pub struct Cmd {
    /// Path to configuration file. Default is $HOME/.unifi-doorbell-chime/config.yaml
    #[structopt(short, long)]
    config: Option<PathBuf>,
}

impl Cmd {
    pub async fn execute(&self) -> Result<()> {
        let config_path = config_path::get(self.config.clone());
        let cfg = Configuration::from_path(&config_path)?;
        init_log(&cfg);

        let client = Arc::new(ApiClient::init(&cfg.unifi)?);

        let app_client = web::Data::new(client.clone());
        let app_cfg = web::Data::new(cfg.clone());

        let (s, r) = chan::sync(0);
        let (lis_tx, lis_rx) = mpsc::channel();

        let lis_cfg = Arc::new(cfg.clone());
        thread::spawn(move || {
            let sys = System::new("listener");
            let listener = spawn(listen(lis_cfg, client.clone(), r));
            lis_tx.send(listener).unwrap();
            sys.run()
        });

        let (srv_tx, srv_rx) = mpsc::channel();
        thread::spawn(move || {
            let sys = System::new("http-server");

            let server = HttpServer::new(move || {
                App::new()
                    .app_data(app_cfg.clone())
                    .app_data(app_client.clone())
                    .wrap(Compress::default())
                    .wrap(Logger::exclude_regex(
                        Logger::default(),
                        r".+\.(js|css|ico)",
                    ))
                    .service(api::wellknown_service())
                    .service(api::service())
                    .service(frontend::spa)
            })
            .bind(format!("0.0.0.0:{}", cfg.server.port))?
            .workers(cfg.server.worker)
            .shutdown_timeout(10)
            .run();

            let _ = srv_tx.send(server);
            sys.run()
        });

        let srv = srv_rx.recv().unwrap();
        lis_rx.recv().unwrap();
        srv.await?;
        drop(s);

        Ok(())
    }
}

fn init_log(c: &Configuration) {
    let level: LevelFilter = match c.log.level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "off" => LevelFilter::Off,
        _ => LevelFilter::Info,
    };
    let _ = env_logger::builder().filter_level(level).try_init();
}
