#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::env;
use dotenv::dotenv;
mod constants;
mod likes;
mod tweets;
mod schema;

async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL don't exist");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("The connection is failed.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
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
