use std::env;
use std::path::PathBuf;

pub fn get(config: Option<PathBuf>) -> PathBuf {
    match config {
        Some(path) => {
            if path.is_absolute() {
                return path;
            }
            env::current_dir().unwrap().join(path)
        }
        None => get_default(),
    }
}

fn get_default() -> PathBuf {
    let base = match dirs::home_dir() {
        Some(path) => path.join(".unifi-doorbell-notifier"),
        None => PathBuf::from("/etc/unifi-doorbell-notifier"),
    };
    base.join("config.yaml")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absolute_path() {
        let path = PathBuf::from("/tmp/config.yaml");
        assert_eq!(get(Some(path.clone())), path);
    }

    #[test]
    fn relative_path() {
        let path = PathBuf::from("tmp/config.yaml");
        assert_ne!(get(Some(path.clone())), path);
    }
}
