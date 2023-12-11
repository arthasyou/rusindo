use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub jwt: rusindo::utility::jwt::JwtCfg,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let c = Config::builder()
            .add_source(config::File::with_name("config/services"))
            .build()?;
        c.try_deserialize()
    }
}
