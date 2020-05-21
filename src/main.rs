use std::{env};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;
use actix_web::{web, App, HttpServer};

mod handlers;
mod graphql;

fn get_port() -> Result<u16, &'static str> {
    match env::var("PORT") {
        Ok(value) => {
            match value.parse::<u16>() {
                Ok(converted) => Ok(converted),
                Err(_) => Err("PORT needs to be a valid number")
            }
        }
        Err(_) => Err("No PORT variable present")
    }
}

#[derive(Clone)]
pub struct AppState {
    host: SocketAddrV4,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let host_port = match get_port() {
        Ok(port) => port,
        Err(e) => {
            println!("FATAL: {}", e);
            std::process::exit(1);
        }
    };

    let host_address = SocketAddrV4::new(
        Ipv4Addr::LOCALHOST,
        host_port,
    );

    let schema = Arc::new(crate::graphql::create_schema());
    let app_state = AppState {
        host: host_address,
    };

    HttpServer::new(move || {
        App::new()
            .data(Arc::clone(&schema))
            .data(app_state.clone())
            .service(handlers::index)
            .service(handlers::graphql)
            .service(handlers::graphiql)
            .default_service(
                web::get().to(handlers::not_found)
            )
    })
        .bind(host_address)?
        .run()
        .await
}