use std::{fs, path::PathBuf};

use serde::{de::DeserializeOwned, ser};

pub fn load_file_and_deserialise<T: DeserializeOwned>(path: &PathBuf) -> eyre::Result<T> {
  let config_content = fs::read_to_string(path)?;
  let cache: T = serde_yaml::from_str(&config_content)?;
  Ok(cache)
}

pub fn save_contents<T: ?Sized + ser::Serialize>(path: &PathBuf, notify: &T) -> eyre::Result<()> {
  fs::write(path, serde_yaml::to_string(notify)?)?;
  Ok(())
}
