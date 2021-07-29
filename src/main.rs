#[macro_use] extern crate rocket;

extern crate uuid;

use scylla::{SessionBuilder, Session};
use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;
use scylla::IntoTypedRows;
use scylla::transport::retry_policy::DefaultRetryPolicy;
use scylla::query::Query;

use uuid::Uuid;

use tokio_retry::Retry;
use tokio_retry::strategy::{ExponentialBackoff, jitter};

use rocket::Request;
use rocket::State;
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

#[get("/hello")]
async fn hello() -> Value {
  json!({
    "status": "success",
    "message": "Hello API!"
  })
}

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    title: String,
    author: String,
    isbn: String
}

#[post("/book", format = "application/json", data = "<book>")]
async fn new_book(book: Json<Book>) -> Value {
    let mut dummy_db: Vec<&Book> = Vec::new();
    let new_book = book.into_inner();
    dummy_db.push(&new_book);
    println!("dummy_db = {:?}", dummy_db);
    json!({ "status": "ok", "id": new_book.isbn })
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

#[derive(FromRow)]
struct VehicleName {
    name: String
}

#[get("/vehicle")]
async fn get_vehicle(session: &State<Session>) -> Value {

    let mut name: String = "unknown".to_string();
    let user_id = Uuid::parse_str("d13fe953-297a-4781-807a-f9becc1b71f6").unwrap();
    let vehicle_id = Uuid::parse_str("60e18f00-34b8-4a52-916c-adbb0204618e").unwrap();

    let retry_strategy = ExponentialBackoff::from_millis(10)
        .map(jitter) // add jitter to delays
        .take(3);    // limit to 3 retries


    let result = Retry::spawn(retry_strategy, || {
        let mut get_vehicle_query: Query = Query::new("SELECT name FROM vehicles.vehicle WHERE user_id = ? and vehicle_id = ?".to_string());
        get_vehicle_query.set_retry_policy(Box::new(DefaultRetryPolicy::new()));
        session.query(get_vehicle_query, (user_id, vehicle_id))
    }).await;

    if let Some(rows) = result
        .expect("Failed to execute query")
        .rows {
            for row in rows.into_typed::<VehicleName>() {
                let vehicle_name: VehicleName = row.expect("Failed to extract VehicleName from Row");
                name = vehicle_name.name;
            }
        }

  json!({
    "vehicle_id": vehicle_id.to_string(),
    "name": name
  })
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let node = "nuckito:9042";

    let session: Session = SessionBuilder::new()
        .known_node(node)
        .retry_policy(Box::new(DefaultRetryPolicy::new()))
        .build()
        .await
        .expect(&format!("Failed to connect {}", node));

    rocket::build()
      .register("/", catchers![not_found])
      .mount("/api", routes![get_vehicle, hello, new_book])
      .manage(session)
      .launch()
      .await
}
