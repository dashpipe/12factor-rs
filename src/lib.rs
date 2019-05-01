#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

extern crate config;
extern crate flexi_logger;
extern crate getopts;
extern crate hyper;
extern crate hyper_router;

mod configuration;
mod logging;
mod monitoring;

use getopts::Options;
use std::env;

pub use configuration::{ConfigProvider, Configuration, ConfigurationPreferences};
pub use logging::LoggingConfig;
pub use monitoring::MonitoringConfig;

pub struct ApplicationBuilder {
    config_prefs: ConfigurationPreferences,
    logging_config: LoggingConfig,
    monitoring_config: MonitoringConfig,
}

impl ApplicationBuilder {
    pub fn default() -> ApplicationBuilder {
        ApplicationBuilder {
            config_prefs: ConfigurationPreferences::default(),
            logging_config: LoggingConfig::default(),
            monitoring_config: MonitoringConfig::default(),
        }
    }

    pub fn with_config_preferences(
        mut self,
        config_prefs: ConfigurationPreferences,
    ) -> ApplicationBuilder {
        self.config_prefs = config_prefs;
        self
    }

    pub fn with_logging_config(mut self, logging_config: LoggingConfig) -> ApplicationBuilder {
        self.logging_config = logging_config;
        self
    }

    pub fn with_monitoring_config(
        mut self,
        monitoring_config: MonitoringConfig,
    ) -> ApplicationBuilder {
        self.monitoring_config = monitoring_config;
        self
    }

    pub fn build(self) -> Application {
        Application::start(&self)
    }
}

pub trait LifeCycle {
    fn shutdown(&self);
}

pub struct Application {
    pub configuration: configuration::Configuration,
}

impl Application {
    fn start(app_builder: &ApplicationBuilder) -> Application {
        let config = Configuration::read_configuration(&app_builder.config_prefs);
        logging::start_logging(&app_builder.logging_config);
        monitoring::start_beacon(&app_builder.monitoring_config);
        Application {
            configuration: config,
        }
    }

    pub fn start_from_cli() -> Application {
        let args: Vec<String> = env::args().collect();
        let mut options = Options::new();
        options.optopt(
            "c",
            "config",
            "fully qualified path to the configuration file",
            "CONFIG",
        );
        options.optopt(
            "s",
            "secrets",
            "fully qualified path to the file that contains secrets",
            "SECRETS",
        );
        let matches = options
            .parse(&args[1..])
            .expect("Unable to read command line");

        let config_prefs = ConfigurationPreferences {
            config_file: matches.opt_str("config"),
            secrets_file: matches.opt_str("secrets"),
            prefix: None,
            separator: None,
        };

        let config = Configuration::read_configuration(&config_prefs);

        let log_config: LoggingConfig = match config.get_value("logging") {
            Ok(v) => v,
            Err(_) => LoggingConfig::default(),
        };

        let montior_config: MonitoringConfig = match config.get_value("monitoring") {
            Ok(v) => v,
            Err(_) => MonitoringConfig::default(),
        };
        logging::start_logging(&log_config);
        monitoring::start_beacon(&montior_config);

        Application {
            configuration: config,
        }
    }
}
