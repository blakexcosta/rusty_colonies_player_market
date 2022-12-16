mod api;
mod models;
mod repository;
mod tests;

#[macro_use]
extern crate rocket;

// This is based off of this tutorial:
// https://dev.to/hackmamba/build-a-rest-api-with-rust-and-mongodb-rocket-version-ah5

// add import below
// use api::user_api::{create_user, get_user, update_user, delete_user, get_all_users}; //import the handler here
use api::user_api::{get_order, create_order, update_order, delete_order,get_account_orders,get_all_orders}; //import the handler here
use api::user_api::hello;
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_order])
        .mount("/", routes![get_order])
        .mount("/", routes![update_order])
        .mount("/", routes![delete_order])
        .mount("/", routes![get_all_orders])
        .mount("/", routes![get_account_orders])
        .mount("/", routes![hello])
}


