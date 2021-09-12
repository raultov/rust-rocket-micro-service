use uuid::Uuid;
use chrono::{Utc, TimeZone, Duration};

use crate::domain::vehicle::Vehicle;
use crate::dto::vehicle_dto::VehicleDTO;

pub fn get_vehicle_dto(vehicle: Vehicle) -> VehicleDTO {
    VehicleDTO {
        name: vehicle.name,
        user_id: vehicle.user_id,
        vehicle_id: Some(vehicle.vehicle_id),
        created_at: Utc.timestamp(vehicle.created_at.num_seconds(), 0),
        vehicle_type: vehicle.vehicle_type,
        retired_at: vehicle.retired_at.map(|d| Utc.timestamp(d.num_seconds(), 0)),
        brand: vehicle.brand,
        model: vehicle.model,
        distance: vehicle.distance,
        owner_since: vehicle.owner_since,
        manufacturing_date: vehicle.manufacturing_date,
        picture: vehicle.picture
    }
}

pub fn get_vehicle(vehicle_dto: VehicleDTO) -> Vehicle {
    Vehicle {
        name: vehicle_dto.name,
        user_id: vehicle_dto.user_id,
        vehicle_id: vehicle_dto.vehicle_id.unwrap_or(Uuid::new_v4()),
        created_at: Duration::seconds(vehicle_dto.created_at.timestamp()),
        vehicle_type: vehicle_dto.vehicle_type,
        retired_at: vehicle_dto.retired_at.map(|d| Duration::seconds(d.timestamp())),
        brand: vehicle_dto.brand,
        model: vehicle_dto.model,
        distance: vehicle_dto.distance,
        owner_since: vehicle_dto.owner_since,
        manufacturing_date: vehicle_dto.manufacturing_date,
        picture: vehicle_dto.picture
    }
}
