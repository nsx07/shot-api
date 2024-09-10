#[macro_use]
extern crate rocket;
use rocket::{Build, Rocket, Request};
use rocket::serde::json::{Json, Value, json};

mod database;
mod models;
mod schema;
mod controller;

#[catch(500)]
fn internal_error() -> Json<Value> {
    Json(json!({
        "error": "Internal Server Error",
        "message": "Whoops! Looks like we messed up."
    }))
}

#[catch(404)]
fn not_found(req: &Request) -> Json<Value> {
    Json(json!({
        "error": "Not Found",
        "message": format!("I couldn't find '{}'. Try something else?", req.uri())
    }))
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![
        controller::index,
        controller::read,
        controller::create,
        controller::update,
        controller::delete
    ]).register("/", catchers![internal_error, not_found])
}