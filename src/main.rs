#[macro_use] extern crate rocket;

use rocket::Request;
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

#[get("/hello")]
fn hello() -> Value {
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
fn new_book(book: Json<Book>) -> Value {
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

#[launch]
fn rocket() -> _ {
    rocket::build()
    .register("/", catchers![not_found])
    .mount("/api", routes![hello, new_book])
}
