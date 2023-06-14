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
    // initialization to connect to a new mongo instance, make sure to supply
    // db and col
    pub fn init(db_name: &str, collection_name: &str) -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap(); // gets our connection, parse from MONGOURI
        // TODO: Rework db and col to no longer be hardcoded strings, and taken in as arguments
        let db = client.database(db_name);//"colony"); // gets a handle to rustDB database
        let col: Collection<Order> = db.collection(collection_name);//"market"); // get the db market collection
        MongoRepo { col }
    }

    //  GET: gets all orders in the database
    pub fn get_all_orders(&self) -> Result<Vec<Order>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of orders");
        let orders = cursors.map(|doc| doc.unwrap()).collect();
        Ok(orders)
    }

    /// GET: Used to get all orders for a specific account
    pub fn get_all_account_orders(&self, name: &String) -> Result<Vec<Order>, Error> {
        // let account_name = ObjectId::parse_str(account_name).unwrap();
        let filter = doc! {"order_poster_account": name};
        let cursors = self
            .col
            .find(filter, None)
            .ok()
            .expect("Error getting list of orders");
        let orders = cursors.map(|doc| doc.unwrap()).collect();
        Ok(orders)
    }


    /// GET: Used to get all BUY orders for a specific account
    /// Order is will either be "buy" or "sell" in lowercase
    pub fn get_all_buy_sell_orders(&self, order: &String) -> Result<Vec<Order>, Error> {
        // let account_name = ObjectId::parse_str(account_name).unwrap();
        let filter = doc! {"order_type": order.to_lowercase()};
        let cursors = self
            .col
            .find(filter, None)
            .ok()
            .expect("Error getting list of orders");
        let orders = cursors.map(|doc| doc.unwrap()).collect();
        Ok(orders)
    }


    // GET: Query the Mongo instance to get an order
    pub fn get_order(&self, id: &String) -> Result<Order, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let order_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting order details");
        Ok(order_detail.unwrap())
    }


    // POST: Creates a new order in the database
    pub fn create_order(&self, new_order: Order) -> Result<InsertOneResult, Error> {
        let new_doc = Order {
            id: None, // tells mongoDB to auto generate order's id
            item_name: new_order.item_name,
            item_number: new_order.item_number,
            order_note: new_order.order_note,
            order_poster_account: new_order.order_poster_account,
            order_type: new_order.order_type,
            price: new_order.price,
            // item_name: new_order.item_name,
            // account_name: new_order.account_name,
            // item_number: new_order.item_number,
            // order_amount: new_order.order_amount,
            // type_order: new_order.type_order,
            // order_note: new_order.order_note,
        };
        let order = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating order");
        Ok(order)
    }


    // PUT: Function to update an order
    pub fn update_order(&self, id: &String, updated_order: Order) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    // commented out "id" because otherwise an new "id" field is appended to the mongo document
                    // "id": updated_order.id,
                    "item_name": updated_order.item_name,
                    "item_number": updated_order.item_number,
                    "order_note": updated_order.order_note,
                    "order_poster_account": updated_order.order_poster_account,
                    "order_type": updated_order.order_type,
                    "price": updated_order.price,
                    // "order_amount": updated_order.order_amount,
                    // "type_order": updated_order.type_order,
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating order");
        Ok(updated_doc)
    }


    // Delete an order, takes in an id
    pub fn delete_order(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let order_detail = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting order");
        Ok(order_detail)
    }
}