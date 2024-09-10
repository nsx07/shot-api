use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use chrono;
use crate::schema::shot;

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Shot {
    pub id: i32,
    pub lat: String,
    pub lon: String,
    pub name: String,
    pub image64: String,
    pub location: String,
    pub hashtags: String,
    pub description: String,
    pub created_at: Option<chrono::NaiveDateTime>
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[table_name = "shot"]
pub struct ShotInput {
    pub lat: String,
    pub lon: String,
    pub name: String,
    pub image64: String,
    pub location: String,
    pub hashtags: String,
    pub description: String,
    pub created_at: Option<chrono::NaiveDateTime>
}