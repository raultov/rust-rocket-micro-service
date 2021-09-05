use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum VehicleType {
    bike
}