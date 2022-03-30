mod config;
mod handlers;

use dotenv::dotenv;
use actix_web::{HttpServer,App,web};

use crate::handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    println!("Starting server at http://{}:{}/", config.server.host, config.server.port);

    HttpServer::new( move|| {
        App::new()
            .route("/hello{_:/?}", web::get().to(hello))
            .route("/hello/{name}{_:/?}", web::get().to(hello_name))
            .route("/hello{_:/?}", web::post().to(hello_post_name))
            })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
