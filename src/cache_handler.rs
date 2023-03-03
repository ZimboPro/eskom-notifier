use std::{fs, path::PathBuf};

use directories_next::ProjectDirs;
use serde::{de::DeserializeOwned, ser};

use crate::CONFIG_FILE;

const QUALIFY_NAME: &str = "io";
const ORGANIZATION_NAME: &str = "South Africa";
const APPLICATION: &str = "Eskom Notifier";

pub fn load_file_and_deserialise<T: DeserializeOwned>(path: &PathBuf) -> eyre::Result<T> {
  let config_content = fs::read_to_string(path)?;
  let cache: T = serde_yaml::from_str(&config_content)?;
  Ok(cache)
}

pub fn save_contents<T: ?Sized + ser::Serialize>(path: &PathBuf, notify: &T) -> eyre::Result<()> {
  if !path.exists() {
    fs::create_dir_all(path.parent().unwrap())?;
  }
  fs::write(path, serde_yaml::to_string(notify)?)?;
  Ok(())
}

pub fn save_state<T: ser::Serialize>(state: &T) {
  if let Some(config) = ProjectDirs::from(QUALIFY_NAME, ORGANIZATION_NAME, APPLICATION) {
    let t = config.config_dir().join(CONFIG_FILE);
    if let Err(e) = save_contents(&t, &state) {
      eprintln!("Saving state error: {}", e)
    }
  }
}

pub fn read_cache<T: DeserializeOwned>() -> eyre::Result<T> {
  let p = ProjectDirs::from(QUALIFY_NAME, ORGANIZATION_NAME, APPLICATION).unwrap();
  let t = p.config_dir().join(CONFIG_FILE);
  load_file_and_deserialise(&t)
}
