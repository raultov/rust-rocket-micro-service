use chrono::offset;
use uuid::Uuid;

use crate::domain::vehicle::Vehicle;
use crate::dto::vehicle_dto::VehicleDTO;
use crate::domain::vehicle_type::VehicleType::bike;

pub fn get_vehicle_dto(vehicle: Vehicle) -> VehicleDTO {
    VehicleDTO {
        name: vehicle.name,
        user_id: vehicle.user_id,
        vehicle_id: Some(vehicle.vehicle_id),
        created_at: offset::Utc::now(),
        vehicle_type: bike,
        retired_at: offset::Utc::now(),
        brand: "the brand".to_string()
    }
}

pub fn get_vehicle(vehicle_dto: VehicleDTO) -> Vehicle {
    Vehicle {
        name: vehicle_dto.name,
        user_id: vehicle_dto.user_id,
        vehicle_id: vehicle_dto.vehicle_id.unwrap_or(Uuid::new_v4())
    }
}
