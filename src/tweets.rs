use crate::constants::APPLICATION_JSON;
use actix_web::web::Path;
use actix_web::{get, post, HttpResponse, Responder};

#[get("/tweets")]
pub async fn get_tweets() -> impl Responder {
    // TODO: Get the tweets
    let tweets = ["tweet1: hi", "tweet2: h2"];
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweets)
}

#[get("/tweets/{id}")]
pub async fn get_tweets_by_id(path: Path<(String,)>) -> impl Responder {
    // TODO: Get the tweets with the id
    let tweet = format!("This is the tweet: {:?}", path.0.as_str());
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweet)
}

#[post("/tweets")]
pub async fn create_tweets() -> impl Responder {
    // TODO: Create the tweets
    let new_tweet = "New Tweet";
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(new_tweet)
}
