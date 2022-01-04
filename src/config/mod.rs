use anyhow::Result;
use config::{Config, Environment, File};
use portpicker::pick_unused_port;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Configuration {
    pub log: Log,
    pub server: Server,
    pub unifi: Unifi,
    pub boot_option: Option<BootOption>,
    pub message: Message,
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
    pub mac_address: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Message {
    pub templates: Vec<String>,
}

impl Configuration {
    pub fn from_path(path: &PathBuf) -> Result<Configuration> {
        if !path.exists() {
            return Err(anyhow::anyhow!("not found config file"));
        }

        let mut cfg = Config::default();
        let default_port: u16 = pick_unused_port().expect("could not find free port");
        cfg.set_default("log.level", "info")
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
        assert_eq!(None, actual.boot_option);
        assert_eq!(
            Message {
                templates: vec!["I'm on my way".to_string(), "I'm busy now".to_string()],
            },
            actual.message
        );
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
            boot_option: Some(BootOption {
                mac_address: "00:00:00:00:00:00".to_string(),
            }),
            message: Message {
                templates: vec!["I'm on my way".to_string(), "I'm busy now".to_string()],
            },
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
