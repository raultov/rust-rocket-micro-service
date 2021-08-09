use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;

#[derive(FromRow)]
pub struct VehicleName {
    pub name: String
}
