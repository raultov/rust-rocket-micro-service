use std::sync::Arc;

use rocket::serde::json::{Json, Value, json};
use rocket::State;
use rocket::serde::uuid::Uuid;
use mockall_double::double;

use crate::dto::book::Book;
use crate::dto::vehicle_dto::VehicleDTO;

#[double]
use crate::service::vehicle_service::VehicleService;


#[get("/hello")]
pub async fn hello() -> Value {
    json!({
        "status": "success",
        "message": "Hello API!"
    })
}

#[post("/book", format = "application/json", data = "<book>")]
pub async fn new_book(book: Json<Book>) -> Value {
    let mut dummy_db: Vec<&Book> = Vec::new();
    let new_book = book.into_inner();
    dummy_db.push(&new_book);

    println!("dummy_db = {:?}", dummy_db);
    json!({
        "status": "success",
        "message": new_book.isbn
    })
}

#[get("/vehicle/<user_id>/<vehicle_id>")]
pub async fn get_vehicle(vehicle_service: &State<Arc<VehicleService>>, user_id: Uuid, vehicle_id: Uuid) -> Value {

    let name = vehicle_service.get_vehicle_name(user_id, vehicle_id).await;

    json!({
        "vehicle_id": vehicle_id,
        "name": name
    })
}

#[post("/vehicle", format = "application/json", data = "<vehicle_json>")]
pub async fn new_vehicle(vehicle_service: &State<Arc<VehicleService>>, vehicle_json: Json<VehicleDTO>) -> Json<VehicleDTO> {
    let vehicle_dto = vehicle_json.into_inner();

    let result = vehicle_service.save_vehicle(vehicle_dto).await;

    Json(result)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    use rocket::http::ContentType;

    #[test]
    fn when_gets_hello_then_responds_with_json_greetings() {
        let rocket_build = rocket::build().mount("/", routes![hello]);
        let client = Client::tracked(rocket_build).expect("valid rocket instance");

        let response = client.get("/hello").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let json_response = response.into_json::<fixture::JSONResponse>().unwrap();
        assert_eq!(fixture::EXPECTED_RESPONSE_STATUS.to_string(), json_response.status);
        assert_eq!(fixture::EXPECTED_GREETINGS_MESSAGE.to_string(), json_response.message);
    }

    #[test]
    fn when_posts_new_book_then_responds_with_json_new_book_created() {
        let rocket_build = rocket::build().mount("/", routes![new_book]);
        let client = Client::tracked(rocket_build).expect("valid rocket instance");

        let response = client.post("/book")
            .header(ContentType::JSON)
            .body(r#"{ "title": "the title", "author": "the author", "isbn": "1379" }"#)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        let json_response = response.into_json::<fixture::JSONResponse>().unwrap();
        assert_eq!(fixture::EXPECTED_RESPONSE_STATUS.to_string(), json_response.status);
        assert_eq!("1379".to_string(), json_response.message);
    }

    #[test]
    fn when_gets_vehicle_then_responds_with_json_vehicle_data() {
        let mut vehicle_service = VehicleService::default();
        vehicle_service.expect_get_vehicle_name()
            .withf(|user_id: &Uuid, _| user_id == &Uuid::parse_str(fixture::USER_ID_STR).unwrap())
            .withf(|_, vehicle_id: &Uuid| vehicle_id == &Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap())
            .times(1)
            .returning(move |_, _| fixture::EXPECTED_VEHICLE_NAME.to_string())
        ;
        let rocket_build = rocket::build().manage(Arc::new(vehicle_service)).mount("/", routes![get_vehicle]);
        let client = Client::untracked(rocket_build).expect("valid rocket instance");

        let response = client.get(format!("/vehicle/{}/{}", fixture::USER_ID_STR, fixture::VEHICLE_ID_STR)).dispatch();

        assert_eq!(response.status(), Status::Ok);
        let json_response = response.into_json::<fixture::JSONVehicleResponse>().unwrap();
        assert_eq!(fixture::VEHICLE_ID_STR.to_string(), json_response.vehicle_id);
        assert_eq!(fixture::EXPECTED_VEHICLE_NAME.to_string(), json_response.name);
    }

    mod fixture {
        use rocket::serde::Deserialize;

        #[derive(Deserialize)]
        pub struct JSONResponse {
            pub status: String,
            pub message: String,
        }


        #[derive(Deserialize)]
        pub struct JSONVehicleResponse {
            pub vehicle_id: String,
            pub name: String,
        }

        pub const EXPECTED_RESPONSE_STATUS: &str = "success";
        pub const EXPECTED_GREETINGS_MESSAGE: &str = "Hello API!";

        pub const USER_ID_STR: &str = "6176bc4b-33b6-4c9c-a4ad-c65da1322a80";
        pub const VEHICLE_ID_STR: &str = "88573010-cf4c-490e-9d29-f8517dc60b90";
        pub const EXPECTED_VEHICLE_NAME: &str = "the vehicle name";
    }
}
