#[cfg(test)]
mod tests {
    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;



    #[test]
    fn main_page() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!("/")).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Welcome to the Rusty Player Colony Market API!");
    }
    
    #[test]
    fn sample_test() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

}