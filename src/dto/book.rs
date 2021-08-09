use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String
}
