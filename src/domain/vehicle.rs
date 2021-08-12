use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;

#[derive(FromRow, Debug)]
pub struct Vehicle {
    pub name: String
}
