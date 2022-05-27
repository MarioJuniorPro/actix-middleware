use std::env;
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> String {

    format!("Welcome from {}:{}", env::var("HOST").unwrap(), env::var("PORT").unwrap())
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();


    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let host = env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("PORT")
    .unwrap_or_else(|_| 8080.to_string())
    .parse::<u16>()
    .unwrap();


    println!("Starting server on {}:{}", host, port);
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            
    })
    .workers(2)
    .bind(format!("{}:{}", host, port))?
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