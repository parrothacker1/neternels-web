use actix_web::{web,App,HttpServer,middleware::Logger};
use env_logger;
use log;
mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address:String="127.0.0.1:8080".to_string();
    env_logger::init();
    log::info!("Starting web server");
    HttpServer::new(move || {
        App::new()
            .route("/api/kernel",web::get().to(handler::list_kernel))
            .route("/api/kernel",web::post().to(handler::add_kernel))
            .route("/api/kernel",web::put().to(handler::update_kernel))
            .route("/api/kernel",web::delete().to(handler::delete_kernel))
            .route("/api/request",web::get().to(handler::list_request))
            .route("/api/request",web::post().to(handler::add_request))
            .route("/api/request",web::put().to(handler::update_request))
            .route("/api/request",web::delete().to(handler::delete_request))
            .route("/api/login",web::post().to(handler::login))
            .wrap(Logger::new("%a %r StatusCode:%s"))
    })
    .bind(address)?
    .run()
    .await
}