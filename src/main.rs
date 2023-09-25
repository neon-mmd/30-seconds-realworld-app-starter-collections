use std::{
    error::Error,
    net::Ipv4Addr,
};

use actix_web::{
    middleware::Logger,
    App, HttpServer,
};
use coi::container;
use stores::memory::TodoMemoryProvider;

mod rest;
mod store_interface;
mod schemas;
mod stores {
    pub mod memory;
}
mod tests {
    pub mod memory;
    pub mod rest;
}

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init();
    let containers = container!{
        repository => TodoMemoryProvider; singleton,
    };

    HttpServer::new(move || {
        // This factory closure is called on each worker thread independently.
        App::new()
            .app_data(containers.clone())
            .wrap(Logger::default())
            .configure(rest::configure())
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}

