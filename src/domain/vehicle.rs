use rocket::serde::uuid::Uuid;
use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;
use chrono::{Duration, NaiveDate};

#[derive(FromRow, Debug)]
pub struct Vehicle {
    pub name                : String,
    pub user_id             : Uuid,
    pub vehicle_id          : Uuid,
    pub created_at          : Duration,
    pub vehicle_type        : String,
    pub retired_at          : Option<Duration>,
    pub brand               : String,
    pub model               : String,
    pub distance            : i32,
    pub owner_since         : NaiveDate,
    pub manufacturing_date  : NaiveDate,
    pub picture             : Option<String>
}
