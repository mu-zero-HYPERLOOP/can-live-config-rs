use can_config_rs::{errors::ConfigError, config::NetworkRef};



#[derive(Debug)]
pub enum LiveConfigError {
    Config(ConfigError),
    Parse(can_yaml_config_rs::errors::Error),
    Reqwest(reqwest::Error),
}

impl From<ConfigError> for LiveConfigError {
    fn from(value: ConfigError) -> Self {
        LiveConfigError::Config(value)
    }
}

impl From<can_yaml_config_rs::errors::Error> for LiveConfigError {
    fn from(value: can_yaml_config_rs::errors::Error) -> Self {
        LiveConfigError::Parse(value)
    }
}

impl From<reqwest::Error> for LiveConfigError {
    fn from(value: reqwest::Error) -> Self {
        LiveConfigError::Reqwest(value)
    }
}

const URL : &'static str =  "https://raw.githubusercontent.com/mu-zero-HYPERLOOP/can-yaml-config-rs/main/test.yaml";

pub fn fetch_live_config() -> Result<NetworkRef, LiveConfigError> {
    // download from github
    let response = reqwest::blocking::get(URL)?;
    let config_str = response.text()?;
    match can_yaml_config_rs::parse_yaml_config(&config_str) {
        Ok(network) => Ok(network),
        Err(err) => Err(err.into()),
    }
}
