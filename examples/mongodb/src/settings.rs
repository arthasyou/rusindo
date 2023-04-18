use config::{Config, ConfigError};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Settings {
    
    pub mongodb: rusindo::database::mongodb::MongoCfg,
    pub web: rusindo::network::web::server::WebCfg,
    
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let c = Config::builder()
            .add_source(config::File::with_name("config/db"))
            .build()?;            
        c.try_deserialize()
    }
}