use std::{path::Path, error::Error};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct URLs {
    pub base_url: String,
    pub iiif_base: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub urls : URLs,
    pub data: String
}

impl Config {
    pub fn load(filename: &str) -> Result<Self, Box<dyn Error>> {
        let config_path = Path::new(filename);
        let config_file = std::fs::File::open(config_path)?;
        let config: Config = serde_yaml::from_reader(config_file)?;
        Ok(config)
    }
}