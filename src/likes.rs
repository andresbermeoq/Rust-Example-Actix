use std::str::FromStr;

use actix_web::{get, post, delete, HttpResponse, Responder};
use actix_web::web::{Data, Path};
use chrono::{NaiveDateTime, Utc};
use diesel::{PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;
use crate::constants::APPLICATION_JSON;
use crate::schema::likes::tweet_id;
use super::schema::likes;
#[derive(Queryable, Insertable, Deserialize, Serialize)]
pub struct Like {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub tweet_id: Uuid,
}

impl Like {
    pub fn new(other_tweet_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            tweet_id: other_tweet_id,
        }
    }
}

#[get("/tweets/{id}/likes")]
pub async fn get_likes_by_tweet(path: Path<(String,)>) -> impl Responder {


    // TODO: Get the tweets with the id
    let likes = 100;
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(likes)
}

#[post("/tweets/{id}/likes")]
pub async fn create_likes_by_tweet(path: Path<(String,)>, pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    // TODO: Get the tweets with the id
    use crate::schema::likes::dsl::*;

    let t_id = &path.0;
    let conn = &mut pool.get().expect("The database connections is failed.");

    let like = Like::new(Uuid::from_str(&t_id).unwrap());
    diesel::insert_into(likes).values(&like).execute(conn).unwrap();
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(&like)
}

#[delete("/tweets/{id}/likes")]
pub async fn delete_likes_by_tweet(path: Path<(String,)>) -> impl Responder {
    // TODO: Get the tweets with the id
    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON).await.unwrap()
}
