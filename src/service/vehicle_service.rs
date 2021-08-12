use rocket::serde::uuid::Uuid;

use crate::repository::vehicle_repository::VehicleRepository;
use crate::domain::vehicle::Vehicle;

const UNKNOWN: &str = "unknown";

pub struct VehicleService {
    vehicle_repository: VehicleRepository
}

impl VehicleService {
    pub async fn new(vehicle_repository: VehicleRepository) -> VehicleService {
        VehicleService {
            vehicle_repository
        }
    }

    pub async fn get_vehicle_name(self: &Self, user_id: Uuid, vehicle_id: Uuid) -> String {
        let vehicle: Option<Vehicle> = self.vehicle_repository.get_vehicle_name(user_id, vehicle_id).await;

        match vehicle {
            None => UNKNOWN.to_string(),
            Some(vehicle) => vehicle.name,
        }
    }
}


