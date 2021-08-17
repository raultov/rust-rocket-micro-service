use std::sync::Arc;

use rocket::serde::uuid::Uuid;

use crate::repository::vehicle_repository::VehicleManager;
use crate::domain::vehicle::Vehicle;

const UNKNOWN: &str = "unknown";

#[async_trait]
pub trait VehicleLogic {
    async fn get_vehicle_name(self: &Self, user_id: Uuid, vehicle_id: Uuid) -> String;
}

pub struct VehicleService {
    vehicle_repository: Arc<dyn VehicleManager + Sync + Send + 'static>,
}

impl VehicleService {
    pub fn new(vehicle_repository: Arc<dyn VehicleManager + Sync + Send + 'static>) -> VehicleService {
        VehicleService {
            vehicle_repository
        }
    }
}

#[async_trait]
impl VehicleLogic for VehicleService {
    async fn get_vehicle_name(self: &Self, user_id: Uuid, vehicle_id: Uuid) -> String {
        let vehicle: Option<Vehicle> = self.vehicle_repository.get_vehicle_name(user_id, vehicle_id).await;

        match vehicle {
            None => UNKNOWN.to_string(),
            Some(vehicle) => vehicle.name,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use mockall::{mock, predicate::*};

    macro_rules! aw {
        ($e: expr) => {
            tokio_test::block_on($e)
        };
    }

    mock! {
        VehicleRepository {}

        #[async_trait]
        impl VehicleManager for VehicleRepository {
            async fn get_vehicle_name(&self, user_id: Uuid, vehicle_id: Uuid) -> Option<Vehicle>;
        }
    }

    #[test]
    fn when_get_vehicle_name_then_returns_vehicle_name() {
        let mut vehicle_repository = MockVehicleRepository::new();

        let user_id = Uuid::parse_str(fixture::USER_ID_STR).unwrap();
        let vehicle_id = Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap();

        vehicle_repository.expect_get_vehicle_name()
            .withf(|user_id: &Uuid, _| user_id == &Uuid::parse_str(fixture::USER_ID_STR).unwrap())
            .withf(|_, vehicle_id: &Uuid| vehicle_id == &Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap())
            .times(1)
            .returning(move |_, _| Some(Vehicle {name: fixture::EXPECTED_VEHICLE_NAME.to_string()}))
        ;

        let vehicle_service = VehicleService::new(Arc::new(vehicle_repository));

        let vehicle_name = aw!(vehicle_service.get_vehicle_name(user_id, vehicle_id));

        assert_eq!(fixture::EXPECTED_VEHICLE_NAME, vehicle_name);
    }

    #[test]
    fn given_none_when_get_vehicle_name_then_returns_unknown() {
        let mut vehicle_repository = MockVehicleRepository::new();

        let user_id = Uuid::parse_str(fixture::USER_ID_STR).unwrap();
        let vehicle_id = Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap();

        vehicle_repository.expect_get_vehicle_name()
            .withf(|user_id: &Uuid, _| user_id == &Uuid::parse_str(fixture::USER_ID_STR).unwrap())
            .withf(|_, vehicle_id: &Uuid| vehicle_id == &Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap())
            .times(1)
            .returning(move |_, _| None)
        ;

        let vehicle_service = VehicleService::new(Arc::new(vehicle_repository));

        let vehicle_name = aw!(vehicle_service.get_vehicle_name(user_id, vehicle_id));

        assert_eq!(UNKNOWN.to_string(), vehicle_name);
    }

    mod fixture {
        pub const USER_ID_STR: &str = "a906615e-2e6a-4edb-9377-5a6b8544791b";
        pub const VEHICLE_ID_STR: &str = "88573010-cf4c-490e-9d29-f8517dc60b90";
        pub const EXPECTED_VEHICLE_NAME: &str = "the vehicle name";
    }
}
