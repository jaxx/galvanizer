use std::path::Path;
use config::reader;
use config::types::Config;
use config::error::{ConfigError, ConfigErrorKind};

type ConfigResult = Result<Config, ConfigError>;

pub struct Configuration;

impl Configuration {
    pub fn open(location: &Path) -> Config {
        let config = match read_configuration_file(location) {
            Ok(config) => config,
            Err(why) => panic!(match why.kind {
                ConfigErrorKind::IoError => format!("Can't read configuration file. DetaiL: {}", why.detail.unwrap()),
                ConfigErrorKind::ParseError => format!("Unexpected error while parsing conguration file. Detail: {}", why.detail.unwrap())
            })
        };

        config
    }
}

fn read_configuration_file(path: &Path) -> ConfigResult {
    let config = try!(reader::from_file(path));
    Ok(config)
}