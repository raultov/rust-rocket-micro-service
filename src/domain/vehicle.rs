use rocket::serde::uuid::Uuid;
use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;

#[derive(FromRow, Debug)]
pub struct Vehicle {
    pub name        : String,
    pub user_id     : Uuid,
    pub vehicle_id  : Uuid
}
