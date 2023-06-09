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
use api::order_api::{get_order, create_order, update_order, delete_order,get_account_orders,get_all_orders, get_buy_orders, get_sell_orders}; //import the handler here
use api::order_api::hello;
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    // create a connection to a db named "colony" and a collection named "market"
    let db = MongoRepo::init("colony","market");
    rocket::build()
        .manage(db)
        .mount("/", routes![create_order])
        .mount("/", routes![get_order])
        .mount("/", routes![update_order])
        .mount("/", routes![delete_order])
        .mount("/", routes![get_all_orders])
        .mount("/", routes![get_account_orders])
        .mount("/", routes![get_buy_orders])
        .mount("/", routes![get_sell_orders])
        .mount("/", routes![hello])
}


