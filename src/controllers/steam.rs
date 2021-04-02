use actix_web::{Error, get, HttpResponse, web};
use futures::{future::ok, stream::once};
use actix_web::dev::BodyEncoding;
use actix_web::http::ContentEncoding;

#[get("/stream")]
pub async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .encoding(ContentEncoding::Br)
        .streaming(body)
}
