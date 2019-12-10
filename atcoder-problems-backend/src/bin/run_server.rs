use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use atcoder_problems_backend::server;
use atcoder_problems_backend::server::AppData;
use std::env;
use std::error::Error;

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    let url = env::var("SQL_URL")?;
    let data = AppData::new(url)?;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .configure(|cfg| server::config(cfg, data.clone()))
            .default_service(
                web::resource("").route(web::route().to(HttpResponse::MethodNotAllowed)),
            )
    })
    .bind("0.0.0.0:8080")?
    .start()
    .await?;
    Ok(())
}