use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{get, web};
use actix_web::Result;

use crate::responses::serving_static::ServingStaticRQ;

#[get("/public/{image}")]
pub async fn serving_static(
    info: web::Path<ServingStaticRQ>,
) -> Result<NamedFile> {
    let path: PathBuf = format!("./src/public/{}", info.image).parse().unwrap();
    Ok(NamedFile::open(path)?)
}
