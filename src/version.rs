use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Info {
    version: &'static str,
}

#[get("/version")]
async fn version_number() -> impl Responder {
    HttpResponse::Ok().json(Info {
        version: env!("CARGO_PKG_VERSION"),
    })
}
