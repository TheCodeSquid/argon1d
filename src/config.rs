use std::{collections::HashMap, fs, path::Path};

use anyhow::{bail, Context, Result};
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

const CONFIG_PATH: &str = "/etc/argon1d.toml";

pub fn load() -> Result<Config> {
    let path = Path::new(CONFIG_PATH);
    let config = if !path.exists() {
        Config::default()
    } else {
        let content = fs::read_to_string(path).with_context(|| "Failed to read config file")?;
        toml::from_str(&content).with_context(|| "Failed to parse config file")?
    };

    // validate
    for speed in config.fan.keys() {
        if *speed > 100 {
            bail!("Fan speed should not exceed 100");
        }
    }

    Ok(config)
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_poll")]
    pub poll_time: u64,
    #[serde(default = "default_delay")]
    pub slow_delay: u64,

    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub fan: HashMap<u8, u8>,
}

impl Default for Config {
    fn default() -> Self {
        let mut fan = HashMap::new();
        fan.insert(55, 10);
        fan.insert(60, 55);
        fan.insert(65, 100);

        Self {
            poll_time: default_poll(),
            slow_delay: default_delay(),
            fan,
        }
    }
}

fn default_poll() -> u64 {
    5
}

fn default_delay() -> u64 {
    10
}
