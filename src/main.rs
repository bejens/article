use actix_web::{App, web, HttpServer};
use actix_web::middleware::Logger;
use env_logger;

#[macro_use]
extern crate serde_derive;

mod api;
mod client;
mod dao;
mod entity;
mod errors;
mod service;
mod util;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %t %s %{User-Agent}i"))
            .service(
                web::scope("/api/v1")
                    .service(web::scope("/articles").configure(articles_router))
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

fn articles_router(cfg: &mut web::ServiceConfig) {
    cfg.service(api::article_api::get)
        .service(api::article_api::create)
        .service(api::article_api::update)
        .service(api::article_api::delete)
        .service(api::article_api::list);
}