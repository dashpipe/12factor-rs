use std::collections::HashMap;
use serde::{Deserialize};
use flexi_logger::{Logger, Cleanup, opt_format};

#[derive(Debug, Deserialize)]
pub struct LoggingConfig{
    root_level: String,
    log_to_file: bool,
    directory: Option<String>,
    rotate_size_mb: Option<u32>,
    num_keep_files: Option<u16>,
    loggers: Option<HashMap<String, String>>,
}

impl Default for LoggingConfig{
    fn default() -> Self { 
        LoggingConfig {
            log_to_file: false,
            root_level: "warn".to_string(),
            directory: None,
            rotate_size_mb: None,
            num_keep_files: None,
            loggers: None, 
        }
    }
}

const MEGA_BYTE : u64 = 1048576; //MEGABYTE 

pub fn start_logging(config: &LoggingConfig){
    let root_and_loggers: String = match &config.loggers {
        None => config.root_level.clone(),
        Some (loggers) => {
            let mut accumul: String = config.root_level.clone();
            for (log_name, level) in loggers {
                accumul.push_str(&format!(", {}={}", log_name, level));
            }
            accumul
        }
    };
    let mut logger = Logger::with_env_or_str(root_and_loggers).format(opt_format);
    if config.log_to_file{
        logger = logger.log_to_file();
        let dir = match &config.directory{
            Some(dir) => dir,
            None => panic!("A logging directory needs to be provided to log to a file"),
        };
        let rotate = match &config.rotate_size_mb{
            Some(size) => *size as u64 * MEGA_BYTE,
            None => MEGA_BYTE,            
        };
        let cleanup = match &config.num_keep_files{
            Some(num) => Cleanup::KeepLogFiles(*num as usize),
            None => Cleanup::KeepLogFiles(5),
        };
        logger = logger.directory(dir)
                       .rotate(rotate as usize, cleanup);
    }
    logger.start().expect("Unable to configure logging!!");
    info!("Logging system started")
}