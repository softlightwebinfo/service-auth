use actix_web::{App, HttpServer, middleware};
use actix_web::middleware::Logger;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use env_logger::Env;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use crate::controllers::index::index;
use crate::controllers::steam::stream;
use crate::state::app_state::AppState;

mod controllers;
mod state;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::Compress::default())
            .service(index)
            .service(stream)
    })
        .keep_alive(75)
        .bind_openssl("127.0.0.1:8000", builder)?
        .run()
        .await
}
