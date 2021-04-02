use actix_web::{App, HttpServer, middleware};

use crate::controllers::index::index;
use crate::controllers::steam::stream;
use crate::state::app_state::AppState;
use actix_web::middleware::Logger;
use env_logger::Env;
use openssl::ssl::{SslFiletype, SslAcceptor, SslMethod};

mod controllers;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    HttpServer::new(|| {
        App::new()
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
