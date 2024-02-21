use actix_web::{
    web, App, HttpResponse, HttpServer, Responder
};
mod tweets;
mod likes;
mod constants;

async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("hello", web::get().to(welcome))
            .service(tweets::get_tweets)
            .service(tweets::get_tweets_by_id)
            .service(likes::get_likes_by_tweet)
            .service(likes::create_likes_by_tweet)
            .service(likes::delete_likes_by_tweet)
            .service(tweets::create_tweets)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
