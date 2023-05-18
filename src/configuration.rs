use eskom_se_push_api::area_info::AreaInfo;
use serde::de::DeserializeOwned;
use std::{fs::File, path::Path};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
  pub token: String,
  pub areas: Vec<AreaInformation>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AreaInformation {
  pub area_id: String,
  pub info: AreaInfo,
}

pub fn load_and_parse_yaml<T: DeserializeOwned>(p: &Path) -> anyhow::Result<T> {
  let t = File::open(p)?;
  let config = serde_yaml::from_reader::<File, T>(t)?;
  Ok(config)
}

pub fn write_to_yaml<T: serde::Serialize>(p: &Path, config: &T) -> anyhow::Result<()> {
  let t = File::create(p)?;
  serde_yaml::to_writer::<File, T>(t, &config)?;
  Ok(())
}
