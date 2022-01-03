mod client;
mod command;
mod config;
mod listener;
mod server;

#[macro_use]
extern crate log;
extern crate chan;
extern crate chan_signal;

#[actix_web::main]
async fn main() {
    if let Some(err) = command::run().await.err() {
        eprintln!("Error: {}", err)
    }
}
