use crate::data::request;
use actix_web::{get, post, web, HttpResponse, Responder};
use log::info;
use rand::Rng;
use crate::db;

#[get("/hello/{name}")]
pub(crate) async fn greet(name: web::Path<String>) -> impl Responder {
    let random: i32 = rand::thread_rng().gen_range(1..=100);
    info!("random number is {random}");
    db::write_to(name.as_str(), random).await;
    format!("Hello Stranger, this is your random number {random}")
}

#[post("/test")]
pub(crate) async fn test(param: web::Json<request::Request>) -> impl Responder {
    if let Ok(result) = serde_json::to_string::<request::Request>(&param) {
        info!("request param is {}", &result);
    }
    HttpResponse::Ok()
        .insert_header(("X-LOG-ID", "123456"))
        .json(&param)
}

#[get("/")]
pub(crate) async fn index() -> impl Responder {
    let random = rand::thread_rng().gen_range(1..=100);
    info!("random number is {random}");
    format!("Hello Stranger, this is your random number {random}")
}
