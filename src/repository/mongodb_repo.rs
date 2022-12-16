use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc}, //modify here
    results::{ InsertOneResult, UpdateResult, DeleteResult}, //modify here
    sync::{Client, Collection},
};
use crate::models::order_model::Order;

pub struct MongoRepo {
    col: Collection<Order>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap(); // gets our connection, parse from MONGOURI
        let db = client.database("player_market_db"); // gets a handle to rustDB database
        let col: Collection<Order> = db.collection("Order"); // get the db user collection
        MongoRepo { col }
    }

    pub fn create_order(&self, new_order: Order) -> Result<InsertOneResult, Error> {
        let new_doc = Order {
            id: None, // tells mongoDB to auto generate user's id
            item_name: new_order.item_name,
            account_name: new_order.account_name,
            item_number: new_order.item_number,
            order_amount: new_order.order_amount,
            type_order: new_order.type_order,
            order_note: new_order.order_note,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub fn get_order(&self, id: &String) -> Result<Order, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub fn update_order(&self, id: &String, new_order: Order) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_order.id,
                    "item_name": new_order.item_name,
                    "account_name": new_order.account_name,
                    "item_number": new_order.item_number,
                    "order_amount": new_order.order_amount,
                    "type_order": new_order.type_order,
                    "order_note": new_order.order_note
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub fn delete_order(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }

    pub fn get_all_orders(&self) -> Result<Vec<Order>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }

    /// Used to get all orders for a specific account
    pub fn get_all_account_orders(&self, account_name: &String) -> Result<Vec<Order>, Error> {
        // let account_name = ObjectId::parse_str(account_name).unwrap();
        let filter = doc! {"account_name": account_name};
        let cursors = self
            .col
            .find(filter, None)
            .ok()
            .expect("Error getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }
}