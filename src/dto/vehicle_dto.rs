use chrono::{DateTime, Utc};
use rocket::serde::uuid::Uuid;
use rocket::serde::{Serialize, Deserialize};
use crate::domain::vehicle_type::VehicleType;

#[derive(Serialize, Deserialize, Debug)]
pub struct VehicleDTO {
    pub name        : String,
    pub user_id     : Uuid,
    pub vehicle_id  : Option<Uuid>,
    pub created_at  : DateTime<Utc>,
    pub vehicle_type: VehicleType,
    pub retired_at  : DateTime<Utc>,
    pub brand       : String
}
