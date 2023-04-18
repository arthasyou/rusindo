use serde::{Deserialize};

use actix_web::{
    App, HttpServer,
    web::{ Data, ServiceConfig },    
};
use actix_cors::Cors;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use actix_web::middleware::{Condition, Logger};


pub async fn start<F, R>(cfg: WebCfg, router: F, services: Vec<R>)
-> std::io::Result<()> 
where 
    F:FnOnce(&mut ServiceConfig) + std::clone::Clone + std::marker::Send + 'static,
    R: Clone + std::marker::Send + 'static,
{
    let server = HttpServer::new(move || {
        let logger = Logger::default();
        let cors = Cors::permissive();
        let mut app = App::new()
            .wrap(Condition::new(cfg.logger, logger))
            .wrap(Condition::new(cfg.cros, cors));
        
        for s in services.iter() {
            app = app.app_data(Data::new(s.clone()));
        }
        app.configure(router.clone())
        
    });

    let newserver = match cfg.ssl {   
        Some(ssl) => {
            let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
            builder
                .set_private_key_file(ssl.key, SslFiletype::PEM)
                .unwrap();
            builder.set_certificate_chain_file(ssl.cert).unwrap();
            server.bind_openssl((cfg.host, cfg.port), builder).unwrap()
        }            
        _ => server.bind((cfg.host, cfg.port)).unwrap()
    };

    newserver
        .run()
        .await    
}

#[derive(Debug, Deserialize)]
pub struct WebCfg {
    pub host: String,
    pub port: u16,
    pub logger: bool,
    pub cros: bool,
    pub ssl: Option<SslOption>,
}

#[derive(Debug, Deserialize)]
pub struct SslOption {
    pub key: String,
    pub cert: String,
}
