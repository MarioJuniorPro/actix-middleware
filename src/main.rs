
use std::env;

use actix_web::dev::{Extensions};
use actix_web::{middleware::*, Responder};
use actix_web::{get, App, HttpServer};
use config::config::Config;
// use serde::Deserialize;
use std::{any::Any, io, net::SocketAddr};
use envy;

use actix_web::{
    rt::net::TcpStream, web, HttpRequest, HttpResponse
    
};

// use config::config::Config;

use crate::middlewares::basic::SayHi;

mod middlewares;
mod config;

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ConnectionInfo {
    bind: SocketAddr,
    peer: SocketAddr,
    ttl: Option<u32>,
}

#[get("/")]
async fn index() -> String {
    let config = envy::from_env::<Config>().unwrap();
    log::info!("Index hit!");
    format!(
        "Welcome from {}", &config.get_bind_addr()
    )
}

fn get_conn_info(connection: &dyn Any, data: &mut Extensions) {
    if let Some(sock) = connection.downcast_ref::<TcpStream>() {
        data.insert(ConnectionInfo {
            bind: sock.local_addr().unwrap(),
            peer: sock.peer_addr().unwrap(),
            ttl: sock.ttl().ok(),
        });
    } else {
        unreachable!("connection should only be plaintext since no TLS is set up");
    }
}

async fn route_whoami(req: HttpRequest) -> impl Responder {
    match req.conn_data::<ConnectionInfo>() {
        Some(info) => HttpResponse::Ok().body(format!(
            "Here is some info about your connection:\n\n{:#?}",
            info
        )),
        None => {
            HttpResponse::InternalServerError().body("Missing expected request extension data")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let config = envy::from_env::<Config>().unwrap();

    env::set_var("RUST_LOG", "debug,simple-auth-server=debug,actix_web=info,actix_server=info");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));


    println!("{:#?}", config);

    log::info!("staring server at http://{}", &config.get_bind_addr());

    HttpServer::new(|| {
        App::new()
            // .wrap(Logger::default())
            .wrap(DefaultHeaders::new().add(("X-Version", "0.2")))
            .wrap(Compress::default())
            .wrap(Logger::default().log_target("http_log"))
            .wrap(SayHi)

            .service(index)
            .service(web::resource("/who").route(web::get().to(route_whoami)))
            
    })
    .on_connect(get_conn_info)
    .workers(2)
    .bind(&config.get_bind_addr())?
    .run()
    .await
}

// Write tests for the main.rs file.
// Language: rust
// Path: src/main.rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
