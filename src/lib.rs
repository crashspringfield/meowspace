#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate validator_derive;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use rocket_contrib::json::JsonValue;
use rocket_cors;
use std::env;
use validator;

mod auth;
mod config;
mod db;
mod errors;
mod models;
mod routes;
mod schema;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found"
    })
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/api",
            routes![
                routes::cats::post_cats_register,
                routes::cats::post_cats_login,
                routes::cats::put_cat,
                routes::cats::get_cat,
                routes::cats::get_all_cats,
            ]
        )
        .attach(db::Conn::fairing())
        .attach(rocket_cors::Cors::default())
        .register(catchers![not_found])
}
