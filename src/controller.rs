use diesel::prelude::*;
use rocket::response::status::{NoContent, Created};
use rocket::serde::json::Json;
use rocket::http::Status;

use crate::database;
use crate::models::{Shot, ShotInput};
use crate::schema::shot::dsl::*;

#[get("/")]
pub fn index() -> Json<Vec<Shot>> {
    let connection = &mut database::establish_connection();
    shot.load::<Shot>(connection).map(Json).expect("Error loading shots")
}

#[post("/", data = "<new_shot>")]
pub fn create(new_shot: Json<ShotInput>) -> Result<Created<Json<Shot>>, Status> {
    let connection = &mut database::establish_connection();
    diesel::insert_into(shot)
        .values(&new_shot.into_inner())
        .execute(connection)
        .map_err(|_| Status::InternalServerError)?;

    let inserted_shot = shot.order(id.desc()).first::<Shot>(connection)
        .map_err(|_| Status::InternalServerError)?;

    Ok(Created::new("/").body(Json(inserted_shot)))
}

#[get("/<shot_id>")]
pub fn read(shot_id: i32) -> Option<Json<Shot>> {
    let connection = &mut database::establish_connection();
    shot.find(shot_id).first::<Shot>(connection).map(Json).ok()
}

#[put("/<shot_id>", data = "<updated_shot>")]
pub fn update(shot_id: i32, updated_shot: Json<ShotInput>) -> Result<Json<Shot>, Status> {
    let connection = &mut database::establish_connection();
    diesel::update(shot.find(shot_id))
        .set(&updated_shot.into_inner())
        .execute(connection)
        .map_err(|_| Status::InternalServerError)?;
    shot.find(shot_id).first::<Shot>(connection).map(Json).map_err(|_| Status::InternalServerError)
}

#[delete("/<shot_id>")]
pub fn delete(shot_id: i32) -> Result<NoContent, Status> {
    let connection = &mut database::establish_connection();
    diesel::delete(shot.find(shot_id))
        .execute(connection)
        .map_err(|_| Status::InternalServerError)?;
    Ok(NoContent)
}