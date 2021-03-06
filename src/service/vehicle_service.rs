use std::sync::Arc;

use rocket::serde::uuid::Uuid;
use mockall::automock;

use crate::repository::vehicle_repository::VehicleRepository;
use crate::mapper::vehicle_mapper;
use crate::domain::vehicle::Vehicle;
use crate::dto::vehicle_dto::VehicleDTO;

const UNKNOWN: &str = "unknown";

pub struct VehicleService {
    vehicle_repository: Arc<dyn VehicleRepository + Sync + Send>,
}

#[automock]
impl VehicleService {
    pub fn new(vehicle_repository: Arc<dyn VehicleRepository+ Sync + Send>) -> VehicleService {
        VehicleService {
            vehicle_repository
        }
    }

    pub async fn get_vehicle_name(&self, user_id: Uuid, vehicle_id: Uuid) -> String {
        let vehicle: Option<Vehicle> = self.vehicle_repository.get_vehicle(user_id, vehicle_id).await;

        match vehicle {
            None => UNKNOWN.to_string(),
            Some(vehicle) => vehicle.name,
        }
    }

    pub async fn save_vehicle(&self, vehicle_dto: VehicleDTO) -> Option<VehicleDTO> {
        let new_vehicle = vehicle_mapper::get_vehicle(vehicle_dto);

        let vehicle = self.vehicle_repository.save_vehicle(new_vehicle).await;

        vehicle.map(|v| vehicle_mapper::get_vehicle_dto(v))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use mockall::mock;
    use chrono::{Duration, NaiveDate, Utc, TimeZone};

    macro_rules! aw {
        ($e: expr) => {
            tokio_test::block_on($e)
        };
    }

    mock! {
        VehicleRepositoryImpl {}

        #[async_trait]
        impl VehicleRepository for VehicleRepositoryImpl {
            async fn get_vehicle(&self, user_id: Uuid, vehicle_id: Uuid) -> Option<Vehicle>;
            async fn save_vehicle(&self, vehicle: Vehicle) -> Option<Vehicle>;
        }
    }

    #[test]
    fn when_get_vehicle_name_then_returns_vehicle_name() {
        let mut vehicle_repository = MockVehicleRepositoryImpl::new();

        let user_id = Uuid::parse_str(fixture::USER_ID_STR).unwrap();
        let vehicle_id = Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap();

        let expected_created_at: Duration = Duration::seconds(1000);
        let expected_owner_since: NaiveDate = NaiveDate::from_num_days_from_ce(500);
        let expected_manufacturing_date: NaiveDate = NaiveDate::from_num_days_from_ce(400);

        vehicle_repository.expect_get_vehicle()
            .withf(|user_id: &Uuid, _| user_id == &Uuid::parse_str(fixture::USER_ID_STR).unwrap())
            .withf(|_, vehicle_id: &Uuid| vehicle_id == &Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap())
            .times(1)
            .returning(move |_, _| Some(Vehicle {name: fixture::EXPECTED_VEHICLE_NAME.to_string(), user_id: user_id, vehicle_id: vehicle_id, created_at: expected_created_at,
                                                 vehicle_type: fixture::EXPECTED_VEHICLE_TYPE.to_string(), retired_at: None, brand: fixture::EXPECTED_BRAND.to_string(),
                                                 model: fixture::EXPECTED_MODEL.to_string(), distance: fixture::EXPECTED_DISTANCE, owner_since: expected_owner_since,
                                                 manufacturing_date: expected_manufacturing_date, picture: None}))
        ;

        let vehicle_service = VehicleService::new(Arc::new(vehicle_repository));

        let vehicle_name = aw!(vehicle_service.get_vehicle_name(user_id, vehicle_id));

        assert_eq!(fixture::EXPECTED_VEHICLE_NAME, vehicle_name);
    }

    #[test]
    fn given_none_when_get_vehicle_name_then_returns_unknown() {
        let mut vehicle_repository = MockVehicleRepositoryImpl::new();

        let user_id = Uuid::parse_str(fixture::USER_ID_STR).unwrap();
        let vehicle_id = Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap();

        vehicle_repository.expect_get_vehicle()
            .withf(|user_id: &Uuid, _| user_id == &Uuid::parse_str(fixture::USER_ID_STR).unwrap())
            .withf(|_, vehicle_id: &Uuid| vehicle_id == &Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap())
            .times(1)
            .returning(move |_, _| None)
        ;

        let vehicle_service = VehicleService::new(Arc::new(vehicle_repository));

        let vehicle_name = aw!(vehicle_service.get_vehicle_name(user_id, vehicle_id));

        assert_eq!(UNKNOWN.to_string(), vehicle_name);
    }

    #[test]
    fn when_save_vehicle_then_vehicle_is_stored() {
        let mut vehicle_repository = MockVehicleRepositoryImpl::new();

        vehicle_repository.expect_save_vehicle()
            .withf(|vehicle: &Vehicle| vehicle.name == fixture::EXPECTED_VEHICLE_NAME.to_string())
            .times(1)
            .returning(move |vehicle| Some(vehicle));

        let vehicle_service = VehicleService::new(Arc::new(vehicle_repository));

        let vehicle_dto = VehicleDTO {
            name: fixture::EXPECTED_VEHICLE_NAME.to_string(),
            user_id: Default::default(),
            vehicle_id: Some(Default::default()),
            created_at: Utc.timestamp(fixture::EXPECTED_CREATED_AT, 0),
            vehicle_type: fixture::EXPECTED_VEHICLE_TYPE.to_string(),
            retired_at: None,
            brand: fixture::EXPECTED_BRAND.to_string(),
            model: fixture::EXPECTED_MODEL.to_string(),
            distance: fixture::EXPECTED_DISTANCE,
            owner_since: NaiveDate::from_num_days_from_ce(fixture::EXPECTED_OWNER_SINCE),
            manufacturing_date: NaiveDate::from_num_days_from_ce(fixture::EXPECTED_MANUFACTURING_DATE),
            picture: Some(fixture::EXPECTED_PICTURE.to_string())
        };

        let vehicle_dto_saved = aw!(vehicle_service.save_vehicle(vehicle_dto)).unwrap();

        assert_eq!(vehicle_dto_saved.name, fixture::EXPECTED_VEHICLE_NAME.to_string());
        assert_eq!(vehicle_dto_saved.user_id, Default::default());
        assert_eq!(vehicle_dto_saved.vehicle_id, Some(Default::default()));
        assert_eq!(vehicle_dto_saved.created_at, Utc.timestamp(fixture::EXPECTED_CREATED_AT, 0));
        assert_eq!(vehicle_dto_saved.vehicle_type, fixture::EXPECTED_VEHICLE_TYPE.to_string());
        assert_eq!(vehicle_dto_saved.retired_at, None);
        assert_eq!(vehicle_dto_saved.brand, fixture::EXPECTED_BRAND.to_string());
        assert_eq!(vehicle_dto_saved.model, fixture::EXPECTED_MODEL.to_string());
        assert_eq!(vehicle_dto_saved.distance, fixture::EXPECTED_DISTANCE);
        assert_eq!(vehicle_dto_saved.owner_since, NaiveDate::from_num_days_from_ce(fixture::EXPECTED_OWNER_SINCE));
        assert_eq!(vehicle_dto_saved.manufacturing_date, NaiveDate::from_num_days_from_ce(fixture::EXPECTED_MANUFACTURING_DATE));
        assert_eq!(vehicle_dto_saved.picture, Some(fixture::EXPECTED_PICTURE.to_string()));
    }

    mod fixture {
        pub const USER_ID_STR: &str = "a906615e-2e6a-4edb-9377-5a6b8544791b";
        pub const VEHICLE_ID_STR: &str = "88573010-cf4c-490e-9d29-f8517dc60b90";
        pub const EXPECTED_VEHICLE_NAME: &str = "the vehicle name";
        pub const EXPECTED_VEHICLE_TYPE: &str = "bike";
        pub const EXPECTED_BRAND: &str = "the brand";
        pub const EXPECTED_MODEL: &str = "the model";
        pub const EXPECTED_DISTANCE: i32 = 15;
        pub const EXPECTED_PICTURE: &str = "the picture path";
        pub const EXPECTED_CREATED_AT: i64 = 5;
        pub const EXPECTED_OWNER_SINCE: i32 = 15;
        pub const EXPECTED_MANUFACTURING_DATE: i32 = 15;
    }
}
