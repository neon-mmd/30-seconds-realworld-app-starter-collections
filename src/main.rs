use std::{
    error::Error,
    net::Ipv4Addr,
};

use actix_web::{
    middleware::Logger,
    web::Data,
    App, HttpServer,
};


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

use crate::store_interface::TodoStore;


#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init();

    let data_store: Data<TodoStore> = Data::new(TodoStore::new());

    HttpServer::new(move || {
        // This factory closure is called on each worker thread independently.
        App::new()
            .wrap(Logger::default())
            .configure(rest::configure(data_store.clone()))
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}

