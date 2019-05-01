use std::sync::RwLock;
use serde::{Deserialize};
use config::{Config, File, Environment, ConfigError};

lazy_static!{
    static ref CONFIGURATION: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings
    });
}

pub fn read_config(config_file: Option<String>, secrets_file: Option<String>){
    let paths = match (config_file, secrets_file) {
        (Some(config), Some(secrets)) => vec![File::with_name(&config), File::with_name(&secrets)],
        (Some(config), None) => vec![File::with_name(&config)],
        (None, Some(secrets)) => vec![File::with_name(&secrets)],
        (None, None) => vec![],
    };
    
    CONFIGURATION.write().unwrap()
           .merge(paths).unwrap()
           .merge(Environment::with_prefix("DASH").separator("_")).unwrap();
}

pub fn get_value<'de, T: Deserialize<'de>>(key: &'de str) -> Result<T, ConfigError>{
    CONFIGURATION
    .read()
    .unwrap()
    .get(key)
}