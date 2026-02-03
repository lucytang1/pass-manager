mod db;
mod endpoints;
mod models;
mod schema;

use actix_web::{web, App, HttpServer};
use db::DbPool;
use dotenvy::dotenv;
use std::env;
use env_logger::Env;
use actix_web::middleware::Logger;
use actix_cors::Cors;
use endpoints::{auth, get_salt, register, get_vault};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    let pool: DbPool = db::establish_connection_pool(&database_url);

    log::info!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(move || {
        let cors = Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header();
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            .service(register::register)
            .service(auth::auth)
            .service(get_vault::get_vault)
            .service(get_salt::get_salt)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
