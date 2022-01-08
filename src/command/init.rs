use anyhow::{ensure, Result};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::command::config_path;

/// Generate configuration template if not exists
#[derive(StructOpt)]
pub struct Cmd {
    /// Path to configuration file. Default is $HOME/.unifi-doorbell-chime/config.yaml
    #[structopt(short, long)]
    config: Option<PathBuf>,
}

const DEFAULT_CONFIG: &str = r#"---
unifi:
  ip: 192.168.1.1
  username: username
  password: password
#boot_option:
#  mac_address: 00:00:00:00:00:00
message:
  templates:
    - I'm on my way
    - I'm busy now
"#;

impl Cmd {
    pub fn execute(&self) -> Result<()> {
        let config_path = config_path::get(self.config.clone());
        ensure!(!config_path.exists(), "config file is already exists");

        let dir = config_path.parent().unwrap();
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }
        let mut f = File::create(config_path.as_path())?;
        f.write_all(DEFAULT_CONFIG.as_bytes())?;
        println!("created config file at {}", config_path.to_str().unwrap());
        Ok(())
    }
}
