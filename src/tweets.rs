use super::schema::tweets;
use crate::constants::APPLICATION_JSON;
use actix_web::web::{Data, Path};
use actix_web::{get, post, HttpResponse, Responder};
use chrono::{NaiveDateTime, Utc};
use diesel::query_dsl::methods::{LimitDsl, OrderDsl};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{ExpressionMethods, PgConnection};
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Queryable, Insertable, Deserialize, Serialize)]
struct Tweet {
    id: Uuid,
    created_at: NaiveDateTime,
    message: String,
}

impl Tweet {
    fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            message,
        }
    }
}

#[get("/tweets")]
pub async fn get_tweets(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    use crate::schema::tweets::dsl::*;

    let conn = &mut pool.get().expect("The database connections is failed.");
    let result = tweets.order(created_at.desc()).limit(10).load::<Tweet>(conn);

    let response = match result {
        Ok(tws) => tws,
        Err(_) => vec![],
    };


    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(response)
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
pub async fn create_tweets(
    req_body: String,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    // TODO: Create the tweets
    let new_tweet = Tweet::new(req_body);
    let conn = &mut pool.get().expect("The database connections is failed.");

    diesel::insert_into(tweets::table).values(&new_tweet).execute(conn).expect("Error in the insert.");

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(&new_tweet)
}
