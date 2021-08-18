use rocket::Request;

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

#[catch(500)]
pub fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn when_not_found_then_responds_with_404() {
        let rocket_build = rocket::build().register("/", catchers![not_found]);
        let client = Client::tracked(rocket_build).expect("valid rocket instance");

        let response = client.get("/unexisting_path").dispatch();

        assert_eq!(response.status(), Status::NotFound);
        let str_response = response.into_string().unwrap();
        assert_eq!(fixture::EXPECTED_NOT_FOUND_RESPONSE.to_string(), str_response);
    }

    #[test]
    fn when_internal_error_then_responds_with_500() {
        let rocket_build = rocket::build()
            .register("/", catchers![internal_error])
            .mount("/", routes![fixture::hello]);
        let client = Client::tracked(rocket_build).expect("valid rocket instance");

        let response = client.get("/hello").dispatch();

        assert_eq!(response.status(), Status::InternalServerError);
        let str_response = response.into_string().unwrap();
        assert_eq!(fixture::EXPECTED_INTERNAL_SERVER_ERROR_RESPONSE.to_string(), str_response);
    }

    mod fixture {
        pub const EXPECTED_NOT_FOUND_RESPONSE: &str = "Oh no! We couldn't find the requested path '/unexisting_path'";
        pub const EXPECTED_INTERNAL_SERVER_ERROR_RESPONSE: &str = "Whoops! Looks like we messed up.";

        #[get("/hello")]
        pub async fn hello() {
            panic!("internal server error");
        }
    }
}
