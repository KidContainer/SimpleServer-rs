use actix_web::{middleware, web, App, HttpServer};
mod data;
mod db;
mod handler;
mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    utils::init_logger();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(handler::index)
            .service(
                web::scope("/api")
                    .service(handler::greet)
                    .service(handler::test),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
