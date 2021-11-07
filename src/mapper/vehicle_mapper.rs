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

#[cfg(test)]
pub mod tests {
    use super::*;
    use chrono::{Duration, NaiveDate, Utc, TimeZone};

    #[test]
    fn given_vehicle_when_get_vehicle_dto_then_returns_vehicle_dto() {

        let vehicle: Vehicle = Vehicle {
            name: fixture::EXPECTED_VEHICLE_NAME.to_string(),
            user_id: Uuid::parse_str(fixture::USER_ID_STR).unwrap(),
            vehicle_id: Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap(),
            created_at: Duration::seconds(fixture::EXPECTED_CREATED_AT),
            vehicle_type: fixture::EXPECTED_VEHICLE_TYPE.to_string(),
            retired_at: Some(Duration::seconds(fixture::EXPECTED_RETIRED_AT)),
            brand: fixture::EXPECTED_BRAND.to_string(),
            model: fixture::EXPECTED_MODEL.to_string(),
            distance: fixture::EXPECTED_DISTANCE,
            owner_since: NaiveDate::from_num_days_from_ce(fixture::EXPECTED_OWNER_SINCE),
            manufacturing_date: NaiveDate::from_num_days_from_ce(fixture::EXPECTED_MANUFACTURING_DATE),
            picture: Some(fixture::EXPECTED_PICTURE.to_string())
        };

        let vehicle_dto = get_vehicle_dto(vehicle);

        assert_eq!(vehicle_dto.name, fixture::EXPECTED_VEHICLE_NAME.to_string());
        assert_eq!(vehicle_dto.user_id, Uuid::parse_str(fixture::USER_ID_STR).unwrap());
        assert_eq!(vehicle_dto.vehicle_id.unwrap(), Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap());
        assert_eq!(vehicle_dto.created_at, Utc.timestamp(fixture::EXPECTED_CREATED_AT, 0));
        assert_eq!(vehicle_dto.vehicle_type, fixture::EXPECTED_VEHICLE_TYPE.to_string());
        assert_eq!(vehicle_dto.retired_at.unwrap(), Utc.timestamp(fixture::EXPECTED_RETIRED_AT, 0));
        assert_eq!(vehicle_dto.brand, fixture::EXPECTED_BRAND.to_string());
        assert_eq!(vehicle_dto.model, fixture::EXPECTED_MODEL.to_string());
        assert_eq!(vehicle_dto.distance, fixture::EXPECTED_DISTANCE);
        assert_eq!(vehicle_dto.owner_since, NaiveDate::from_num_days_from_ce(fixture::EXPECTED_OWNER_SINCE));
        assert_eq!(vehicle_dto.manufacturing_date, NaiveDate::from_num_days_from_ce(fixture::EXPECTED_MANUFACTURING_DATE));
        assert_eq!(vehicle_dto.picture.unwrap(), fixture::EXPECTED_PICTURE.to_string());
    }

    #[test]
    fn given_vehicle_dto_when_get_vehicle_then_returns_vehicle() {

        let vehicle_dto: VehicleDTO = VehicleDTO {
            name: fixture::EXPECTED_VEHICLE_NAME.to_string(),
            user_id: Uuid::parse_str(fixture::USER_ID_STR).unwrap(),
            vehicle_id: Some(Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap()),
            created_at: Utc.timestamp(fixture::EXPECTED_CREATED_AT, 0),
            vehicle_type: fixture::EXPECTED_VEHICLE_TYPE.to_string(),
            retired_at: Some(Utc.timestamp(fixture::EXPECTED_RETIRED_AT, 0)),
            brand: fixture::EXPECTED_BRAND.to_string(),
            model: fixture::EXPECTED_MODEL.to_string(),
            distance: fixture::EXPECTED_DISTANCE,
            owner_since: NaiveDate::from_num_days_from_ce(fixture::EXPECTED_OWNER_SINCE),
            manufacturing_date: NaiveDate::from_num_days_from_ce(fixture::EXPECTED_MANUFACTURING_DATE),
            picture: Some(fixture::EXPECTED_PICTURE.to_string())
        };

        let vehicle = get_vehicle(vehicle_dto);

        assert_eq!(vehicle.name, fixture::EXPECTED_VEHICLE_NAME.to_string());
        assert_eq!(vehicle.user_id, Uuid::parse_str(fixture::USER_ID_STR).unwrap());
        assert_eq!(vehicle.vehicle_id, Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap());
        assert_eq!(vehicle.created_at, Duration::seconds(fixture::EXPECTED_CREATED_AT));
        assert_eq!(vehicle.vehicle_type, fixture::EXPECTED_VEHICLE_TYPE.to_string());
        assert_eq!(vehicle.retired_at.unwrap(), Duration::seconds(fixture::EXPECTED_RETIRED_AT));
        assert_eq!(vehicle.brand, fixture::EXPECTED_BRAND.to_string());
        assert_eq!(vehicle.model, fixture::EXPECTED_MODEL.to_string());
        assert_eq!(vehicle.distance, fixture::EXPECTED_DISTANCE);
        assert_eq!(vehicle.owner_since, NaiveDate::from_num_days_from_ce(fixture::EXPECTED_OWNER_SINCE));
        assert_eq!(vehicle.manufacturing_date, NaiveDate::from_num_days_from_ce(fixture::EXPECTED_MANUFACTURING_DATE));
        assert_eq!(vehicle.picture.unwrap(), fixture::EXPECTED_PICTURE.to_string());
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
        pub const EXPECTED_RETIRED_AT: i64 = 5000;
        pub const EXPECTED_OWNER_SINCE: i32 = 15;
        pub const EXPECTED_MANUFACTURING_DATE: i32 = 15;
    }
}
