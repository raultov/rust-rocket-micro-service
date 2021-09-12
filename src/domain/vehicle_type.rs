use rocket::serde::{Serialize, Deserialize};
use scylla::macros::{FromUserType, IntoUserType};
use scylla::cql_to_rust::FromCqlVal;

#[derive(Serialize, Deserialize, Debug, IntoUserType, FromUserType)]
pub enum VehicleType {
    bike
}