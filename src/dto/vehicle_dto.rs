use chrono::{NaiveDate, DateTime, Utc};
use rocket::serde::uuid::Uuid;
use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VehicleDTO {
    pub name                : String,
    pub user_id             : Uuid,
    pub vehicle_id          : Option<Uuid>,
    pub created_at          : DateTime<Utc>,
    pub vehicle_type        : String,
    pub retired_at          : Option<DateTime<Utc>>,
    pub brand               : String,
    pub model               : String,
    pub distance            : i32,
    pub owner_since         : NaiveDate,
    pub manufacturing_date  : NaiveDate,
    pub picture             : Option<String>
}
