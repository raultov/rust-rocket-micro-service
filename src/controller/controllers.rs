use crate::Arc;

use rocket::serde::json::{Json, Value, json};
use rocket::State;
use rocket::serde::uuid::Uuid;

use crate::dto::book::Book;
use crate::service::vehicle_service::VehicleLogic;

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
    json!({ "status": "ok", "id": new_book.isbn })
}

#[get("/vehicle/<user_id>/<vehicle_id>")]
pub async fn get_vehicle(vehicle_logic: &State<Arc<dyn VehicleLogic + Sync + Send + 'static>>, user_id: Uuid, vehicle_id: Uuid) -> Value {

  let name = vehicle_logic.get_vehicle_name(user_id, vehicle_id).await;

  json!({
    "vehicle_id": vehicle_id,
    "name": name
  })
}

