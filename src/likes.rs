use actix_web::{get, post, delete, HttpResponse, Responder};
use actix_web::web::Path;
use crate::constants::APPLICATION_JSON;

#[get("/tweets/{id}/likes")]
pub async fn get_likes_by_tweet(path: Path<(String,)>) -> impl Responder {
    // TODO: Get the tweets with the id
    let likes = 100;
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(likes)
}

#[post("/tweets/{id}/likes")]
pub async fn create_likes_by_tweet(path: Path<(String,)>) -> impl Responder {
    // TODO: Get the tweets with the id
    let like = "OK";
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(like)
}

#[delete("/tweets/{id}/likes")]
pub async fn delete_likes_by_tweet(path: Path<(String,)>) -> impl Responder {
    // TODO: Get the tweets with the id
    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON).await.unwrap()
}
