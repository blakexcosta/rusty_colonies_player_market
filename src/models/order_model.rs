use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub item_name: String,
    pub item_number: i32,
    pub order_note: String,
    pub order_poster_account: String,
    pub order_type: String,
    pub price: i32,
    // pub item_name: String,
    // pub account_name: String,
    // pub item_number: i64,
    // pub order_amount: i64,
    // pub type_order: String,
}