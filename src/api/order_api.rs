use crate::{models::order_model::Order, repository::mongodb_repo::MongoRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult}; //modify here
use rocket::{http::Status, serde::json::Json, State};

    /// Used to welcome to the API if they hit a default endpoint
    /// # Endpoint Example
    /// `127.0.0.1:8000/` 
    #[get("/")]
    pub fn hello() -> &'static str {
        "Welcome to the Rusty Player Colony Market API!"
    }


    /// GET all orders in the market
    #[get("/orders")]
    pub fn get_all_orders(db: &State<MongoRepo>) -> Result<Json<Vec<Order>>, Status> {
        let orders = db.get_all_orders();
        match orders {
            Ok(orders) => Ok(Json(orders)),
            Err(_) => Err(Status::InternalServerError),
        }
    }


    /// GET all order for a specific account, the account name should be passed in as a paramtere
    /// # Example: 
    /// `127.0.0.1:8000/orders/bcosta`
    #[get("/orders/<account>")]
    pub fn get_account_orders(db: &State<MongoRepo>, account: String) -> Result<Json<Vec<Order>>, Status> {
        let id = account;
        if id.is_empty() {
            return Err(Status::BadRequest);
        };
        let order_detail = db.get_all_account_orders(&id);
        match order_detail {
            Ok(order) => Ok(Json(order)),
            Err(_) => Err(Status::InternalServerError),
        }
    }


    /// GET all BUY orders in the database
    /// # Example: 
    /// `127.0.0.1:8000/orders/bcosta`
    #[get("/orders/buy")]
    pub fn get_buy_orders(db: &State<MongoRepo>) -> Result<Json<Vec<Order>>, Status> {
        let id = "buy".to_owned();
        if id.is_empty() {
            return Err(Status::BadRequest);
        };
        let order_detail = db.get_all_buy_sell_orders(&id);
        match order_detail {
            Ok(order) => Ok(Json(order)),
            Err(_) => Err(Status::InternalServerError),
        }
    }


    /// GET all SELL orders in the database
    /// # Example: 
    /// `127.0.0.1:8000/orders/bcosta`
    #[get("/orders/sell")]
    pub fn get_sell_orders(db: &State<MongoRepo>) -> Result<Json<Vec<Order>>, Status> {
        let id = "sell".to_owned();
        if id.is_empty() {
            return Err(Status::BadRequest);
        };
        let order_detail = db.get_all_buy_sell_orders(&id);
        match order_detail {
            Ok(order) => Ok(Json(order)),
            Err(_) => Err(Status::InternalServerError),
        }
    }

    /// GET Endpoint for a specific order
    /// # Example:
    /// `127.0.0.1:8000/order/639b5d9b816980de19548091` 
    #[get("/order/<order_id>")]
    pub fn get_order(db: &State<MongoRepo>, order_id: String) -> Result<Json<Order>, Status> {
        let id = order_id;
        if id.is_empty() {
            return Err(Status::BadRequest);
        };
        let order_detail = db.get_order(&id);
        match order_detail {
            Ok(order) => Ok(Json(order)),
            Err(_) => Err(Status::InternalServerError),
        }
    }


    /// POST Endpoint, used to post a new order
    /// # Endpoint Example
    /// `127.0.0.1:8000/` 
    /// # Arguments
    /// takes in a `&State<MongoRepot>` and a `Json<Order>`. `Json<Order>` is a data guard, so make sure json you pass
    /// is in the format of how you specific the model to look in the /models folder
    #[post("/order", data = "<new_order>")]
    pub fn create_order( db: &State<MongoRepo>, new_order: Json<Order>,) -> Result<Json<InsertOneResult>, Status> {
        let data = Order {
            id: None, // tells mongoDB to auto generate order's id
            item_name: new_order.item_name.to_owned(),
            item_number: new_order.item_number.to_owned(),
            order_note: new_order.order_note.to_owned(),
            order_poster_account: new_order.order_poster_account.to_owned(),
            order_type: new_order.order_type.to_owned(),
            price: new_order.price.to_owned(),
            // item_name: new_order.item_name.to_owned(),
            // account_name: new_order.account_name.to_owned(),
            // item_number: new_order.item_number.to_owned(),
            // order_amount: new_order.order_amount.to_owned(),
            // type_order: new_order.type_order.to_owned(),
            // order_note: new_order.order_note.to_owned(),
        };
        let order_detail = db.create_order(data);
        match order_detail {
            Ok(order) => Ok(Json(order)),
            Err(_) => Err(Status::InternalServerError),
        }
    }


    /// PUT Endpoint for a specific order
    /// # Example:
    /// `127.0.0.1:8000/order/639b5d9b816980de19548091
    /// 
    /// This will update any order you specify with the id in the url
    #[put("/order/<existing_order_id>", data = "<updated_order>")]
    pub fn update_order( db: &State<MongoRepo>, existing_order_id: String, updated_order: Json<Order>,) -> Result<Json<Order>, Status> {
        let id = existing_order_id;
        if id.is_empty() {
            return Err(Status::BadRequest);
        };
        let data = Order {
            id: Some(ObjectId::parse_str(&id).unwrap()), // get the id passed
            item_name: updated_order.item_name.to_owned(),
            item_number: updated_order.item_number.to_owned(),
            order_note: updated_order.order_note.to_owned(),
            order_poster_account: updated_order.order_poster_account.to_owned(),
            order_type: updated_order.order_type.to_owned(),
            price: updated_order.price.to_owned(),
            // item_name: updated_order.item_name.to_owned(),
            // account_name: updated_order.account_name.to_owned(),
            // item_number: updated_order.item_number.to_owned(),
            // order_amount: updated_order.order_amount.to_owned(),
            // type_order: updated_order.type_order.to_owned(),
            // order_note: updated_order.order_note.to_owned(),
        };
        let update_result = db.update_order(&id, data);
        match update_result {
            Ok(update) => {
                if update.matched_count == 1 {
                    let updated_order_info = db.get_order(&id);
                    return match updated_order_info {
                        Ok(order) => Ok(Json(order)),
                        Err(_) => Err(Status::InternalServerError),
                    };
                } else {
                    return Err(Status::NotFound);
                }
            }
            Err(_) => Err(Status::InternalServerError),
        }
    }


    /// DELETE Endpoint for a specific order
    /// # Example: 
    /// `127.0.0.1:8000/order/639b5d9b816980de19548091`
    #[delete("/order/<order_id>")]
    pub fn delete_order(db: &State<MongoRepo>, order_id: String) -> Result<Json<&str>, Status> {
        let id = order_id;
        if id.is_empty() {
            return Err(Status::BadRequest);
        };
        let result = db.delete_order(&id);
        match result {
            Ok(res) => {
                if res.deleted_count == 1 {
                    return Ok(Json("Order deleted!"));
                } else {
                    return Err(Status::NotFound);
                }
            }
            Err(_) => Err(Status::InternalServerError),
        }
    }