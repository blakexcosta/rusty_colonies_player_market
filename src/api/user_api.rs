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


    /// POST Endpoint, used to post a new order
    /// # Endpoint Example
    /// `127.0.0.1:8000/` 
    /// # Arguments
    /// takes in a `&State<MongoRepot>` and a `Json<Order>`. `Json<Order>` is a data guard, so make sure json you pass
    /// is in the format of how you specific the model to look in the /models folder
    #[post("/order", data = "<new_order>")]
    pub fn create_order( db: &State<MongoRepo>, new_order: Json<Order>,) -> Result<Json<InsertOneResult>, Status> {
        let data = Order {
            id: None, // tells mongoDB to auto generate user's id
            item_name: new_order.item_name.to_owned(),
            account_name: new_order.account_name.to_owned(),
            item_number: new_order.item_number.to_owned(),
            order_amount: new_order.order_amount.to_owned(),
            type_order: new_order.type_order.to_owned(),
            order_note: new_order.order_note.to_owned(),
        };
        let user_detail = db.create_order(data);
        match user_detail {
            Ok(user) => Ok(Json(user)),
            Err(_) => Err(Status::InternalServerError),
        }
    }

    /// GET Endpoint for a specific order
    /// # Example:
    /// `127.0.0.1:8000/order/639b5d9b816980de19548091` 
    #[get("/order/<path>")]
    pub fn get_order(db: &State<MongoRepo>, path: String) -> Result<Json<Order>, Status> {
        let id = path;
        if id.is_empty() {
            return Err(Status::BadRequest);
        };
        let order_detail = db.get_order(&id);
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
    #[put("/order/<path>", data = "<new_order>")]
    pub fn update_order( db: &State<MongoRepo>, path: String, new_order: Json<Order>,) -> Result<Json<Order>, Status> {
        let id = path;
        if id.is_empty() {
            return Err(Status::BadRequest);
        };
        let data = Order {
            id: Some(ObjectId::parse_str(&id).unwrap()),
            item_name: new_order.item_name.to_owned(),
            account_name: new_order.account_name.to_owned(),
            item_number: new_order.item_number.to_owned(),
            order_amount: new_order.order_amount.to_owned(),
            type_order: new_order.type_order.to_owned(),
            order_note: new_order.order_note.to_owned(),
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

    /// DELETE Endpoint for a specific user
    /// # Example: 
    /// `127.0.0.1:8000/order/639b5d9b816980de19548091`
    #[delete("/order/<path>")]
    pub fn delete_order(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
        let id = path;
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

    /// GET all orders in the market
    #[get("/orders")]
    pub fn get_all_orders(db: &State<MongoRepo>) -> Result<Json<Vec<Order>>, Status> {
        let users = db.get_all_orders();
        match users {
            Ok(users) => Ok(Json(users)),
            Err(_) => Err(Status::InternalServerError),
        }
    }

    /// GET all order for a specific account, the account name should be passed in as a paramtere
    /// # Example: 
    /// `127.0.0.1:8000/orders/Blake`
    #[get("/orders/<account_name>")]
    pub fn get_account_orders(db: &State<MongoRepo>, account_name: String) -> Result<Json<Vec<Order>>, Status> {
        let id = account_name;
        if id.is_empty() {
            return Err(Status::BadRequest);
        };
        let order_detail = db.get_all_account_orders(&id);
        match order_detail {
            Ok(order) => Ok(Json(order)),
            Err(_) => Err(Status::InternalServerError),
        }
    }