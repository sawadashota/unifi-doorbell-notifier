use anyhow::{ensure, Result};
use config::{Config, Environment, File};
use portpicker::pick_unused_port;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Configuration {
    pub log: Log,
    pub server: Server,
    pub unifi: Unifi,
    pub boot_option: BootOption,
    pub message: Message,
    pub open_browser: bool,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Server {
    pub port: u16,
    pub worker: usize,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Unifi {
    pub ip: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct BootOption {
    pub mac_address: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Message {
    pub templates: Vec<String>,
}

impl Configuration {
    pub fn from_path(path: &PathBuf) -> Result<Configuration> {
        ensure!(path.exists(), "not found config file");

        let mut cfg = Config::default();
        let default_port: u16 = pick_unused_port().expect("could not find free port");
        cfg.set_default("log.level", "info")
            .unwrap()
            .set_default("open_browser", true)
            .unwrap()
            .set_default("server.worker", 3)
            .unwrap()
            .set_default("server.port", default_port.to_string())
            .unwrap()
            .merge(File::from(path.as_path()))
            .unwrap()
            .merge(Environment::default())
            .unwrap();
        Ok(cfg.try_into::<Configuration>()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_yaml() {
        println!("{}", env!("CARGO_MANIFEST_DIR"));
        let actual = Configuration::from_path(&PathBuf::from("src/config/testdata/valid.yaml"))
            .expect("valid.yaml should be deserialized");

        assert_eq!(
            Log {
                level: "info".to_string(),
            },
            actual.log
        );
        assert_eq!(3, actual.server.worker);
        assert!(actual.server.port > 1000);
        assert_eq!(
            Unifi {
                ip: "192.168.1.1".to_string(),
                username: "username".to_string(),
                password: "password".to_string(),
            },
            actual.unifi
        );
        assert_eq!(
            BootOption {
                mac_address: vec!["00:00:00:00:00:00".to_string()],
            },
            actual.boot_option
        );
        assert_eq!(
            Message {
                templates: vec!["I'm on my way".to_string(), "I'm busy now".to_string()],
            },
            actual.message
        );
        assert_eq!(true, actual.open_browser);
    }

    #[test]
    fn valid_full_options_yaml() {
        let actual = Configuration::from_path(&PathBuf::from(
            "src/config/testdata/valid_full_options.yaml",
        ))
        .expect("valid_full_options.yaml should be deserialized");
        let expected = Configuration {
            log: Log {
                level: "info".to_string(),
            },
            server: Server {
                port: 8080,
                worker: 10,
            },
            unifi: Unifi {
                ip: "192.168.1.1".to_string(),
                username: "username".to_string(),
                password: "password".to_string(),
            },
            boot_option: BootOption {
                mac_address: vec!["00:00:00:00:00:00".to_string()],
            },
            message: Message {
                templates: vec!["I'm on my way".to_string(), "I'm busy now".to_string()],
            },
            open_browser: false,
        };
        assert_eq!(expected, actual)
    }

    #[test]
    #[should_panic]
    fn not_found() {
        Configuration::from_path(&PathBuf::from("src/config/testdata/not_found.yaml"))
            .expect("valid.yaml should be deserialized");
    }

    #[test]
    #[should_panic]
    fn invalid_yaml() {
        Configuration::from_path(&PathBuf::from("src/config/testdata/invalid.yaml"))
            .expect("valid.yaml should be deserialized");
    }
}
