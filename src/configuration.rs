use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

pub trait ConfigProvider {
    fn get_value<'de, T: Deserialize<'de>>(&self, key: &'de str) -> Result<T, ConfigError>;
}

pub struct ConfigurationPreferences{
    pub config_file: Option<String>,
    pub secrets_file: Option<String>,
    pub prefix: Option<String>,
    pub separator: Option<String>,
}

impl Default for ConfigurationPreferences{
    fn default() -> Self {
        ConfigurationPreferences{
            config_file: None,
            secrets_file: None,
            prefix: None,
            separator: None,
        }
    }
}

pub struct Configuration {
    configuration: Config,
}

impl Configuration {
    pub fn read_configuration(prefs: &ConfigurationPreferences) -> Configuration{
        let paths = match (prefs.config_file.clone(), prefs.secrets_file.clone()) {
            (Some(config), Some(secrets)) => {
                vec![File::with_name(&config), File::with_name(&secrets)]
            }
            (Some(config), None) => vec![File::with_name(&config)],
            (None, Some(secrets)) => vec![File::with_name(&secrets)],
            (None, None) => vec![],
        };

        let env = match (prefs.prefix.clone(), prefs.separator.clone()){
            (Some(prefix) , Some(separator)) => {
                Environment::with_prefix(&prefix).separator(&separator)
            },
            (Some(prefix), None) => {
                Environment::with_prefix(&prefix)
            },
            (None, Some(separator)) => {
                Environment::default().separator(&separator)
            },
            (None, None) => {
                Environment::default()
            },
        };

        let mut a_config = Config::new();
        a_config.merge(paths).unwrap()
        .merge(env);
        Configuration{
            configuration : a_config,
        }
    }
}

impl ConfigProvider for Configuration{
    fn get_value<'de, T: Deserialize<'de>>(&self, key: &'de str) -> Result<T, ConfigError>{
        self.configuration.get(key)
    }
}

#[cfg(test)]
mod tests{
    
}