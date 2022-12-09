use std::{error::Error, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct URLs {
    #[serde(default = "base_url_default")]
    pub base_url: String,
    #[serde(default = "iiif_base_default")]
    pub iiif_base: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub urls: URLs,
    #[serde(default = "projects_default")]
    pub projects_path: String,
    #[serde(default = "listen_default")]
    pub listen: String,
}

impl Config {
    pub fn load(filename: &str) -> Result<Self, Box<dyn Error>> {
        let config_path = Path::new(filename);
        let config_file = std::fs::File::open(config_path)?;
        let config: Config = serde_yaml::from_reader(config_file)?;
        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            urls: URLs {
                base_url: base_url_default(),
                iiif_base: iiif_base_default(),
            },
            projects_path: projects_default(),
            listen: listen_default(),
        }
    }
}

fn base_url_default() -> String {
    "http://localhost:3000".to_string()
}

fn iiif_base_default() -> String {
    "/iiif/2".to_string()
}

fn projects_default() -> String {
    "./projects".to_string()
}

fn listen_default() -> String {
    "127.0.0.1:3000".to_string()
}
