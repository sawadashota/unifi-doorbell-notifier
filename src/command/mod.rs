mod config_path;
mod init;
mod serve;

use anyhow::Result;
use init::Cmd as Init;
use serve::Cmd as Serve;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(long_version(option_env ! ("LONG_VERSION").unwrap_or(env ! ("CARGO_PKG_VERSION"))))]
#[structopt(name = "Unifi Doorbell Chime")]
enum RootCmd {
    #[structopt(name = "init")]
    Init(Init),
    #[structopt(name = "serve")]
    Serve(Serve),
}

pub async fn run() -> Result<()> {
    match RootCmd::from_args() {
        RootCmd::Init(cmd) => cmd.execute(),
        RootCmd::Serve(cmd) => cmd.execute().await,
    }
}
