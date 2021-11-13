use std::sync::Arc;
use scylla::IntoTypedRows;

use rocket::serde::uuid::Uuid;

use crate::dao::session_manager::SessionManager;
use crate::domain::vehicle::Vehicle;

use chrono::{Utc, TimeZone};

#[async_trait]
pub trait VehicleRepository {
    async fn get_vehicle(&self, user_id: Uuid, vehicle_id: Uuid) -> Option<Vehicle>;
    async fn save_vehicle(&self, vehicle: Vehicle) -> Option<Vehicle>;
}

pub struct VehicleRepositoryImpl {
    queriable: Arc<dyn SessionManager + Sync + Send + 'static>,
}

impl VehicleRepositoryImpl {
    pub fn new(queriable: Arc<dyn SessionManager + Sync + Send + 'static>) -> VehicleRepositoryImpl {
        VehicleRepositoryImpl {
            queriable
        }
    }
}

#[async_trait]
impl VehicleRepository for VehicleRepositoryImpl {
    async fn get_vehicle(&self, user_id: Uuid, vehicle_id: Uuid) ->  Option<Vehicle> {
        let query = format!("SELECT name, user_id, vehicle_id, created_at, vehicle_type, retired_at, brand, model, distance, owner_since, manufacturing_date, picture \
            FROM vehicles.vehicle \
            WHERE user_id = {} and vehicle_id = {}", user_id, vehicle_id);

        let result = self.queriable.execute_query(&query).await;

        if let Some(rows) = result
            .expect(&format!("Failed to execute query {}", query))
            .rows {
                for row in rows.into_typed::<Vehicle>() {
                    let vehicle: Vehicle = row.expect("Failed to extract Vehicle from Row");
                    return Some(vehicle);
                }
            };

        None
    }

    async fn save_vehicle(&self, vehicle: Vehicle) -> Option<Vehicle> {
        let query = format!("INSERT INTO vehicles.vehicle   (user_id,       \
                                            vehicle_id,    \
                                            vehicle_type,  \
                                            name,          \
                                            created_at,    \
                                            retired_at,    \
                                            brand,         \
                                            model,         \
                                            distance,      \
                                            owner_since,   \
                                            manufacturing_date, \
                                            picture) VALUES ({}, {}, '{}', '{}', '{}', {}, '{}', '{}', {}, '{}', '{}', {})",
                            vehicle.user_id, vehicle.vehicle_id, vehicle.vehicle_type, vehicle.name, Utc.timestamp(vehicle.created_at.num_seconds(), 0),
                            vehicle.retired_at.map(|d| format!("'{}'", Utc.timestamp(d.num_seconds(), 0))).unwrap_or_else(|| "null".to_string()),
                            vehicle.brand, vehicle.model, vehicle.distance, vehicle.owner_since, vehicle.manufacturing_date,
                            vehicle.picture.as_ref().map(|p| format!("'{}'", p)).unwrap_or_else(|| "null".to_string())
        );

        let result = self.queriable.execute_query(&query).await;

        match result {
            Ok(_) => Some(vehicle),
            Err(e) => {
                println!("Failed to insert Vehicle {:?} with error {:?}", query, e);
                None
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use scylla::QueryResult;
    use scylla::transport::errors::QueryError;
    use scylla::frame::response::result::CqlValue;
    use chrono::{Duration, NaiveDate};

    use mockall::mock;
    use scylla::cql_to_rust::FromCqlVal;

    macro_rules! aw {
        ($e: expr) => {
            tokio_test::block_on($e)
        };
    }

    mock! {
        SessionManagerImpl {}

        #[async_trait]
        impl SessionManager for SessionManagerImpl {
            async fn execute_query(&self, query_statement: &str) -> Result<QueryResult, QueryError>;
        }
    }

    #[test]
    fn when_get_vehicle_then_returns_vehicle() {
        let mut session_manager = MockSessionManagerImpl::new();

        let user_id = Uuid::parse_str(fixture::USER_ID_STR).unwrap();
        let vehicle_id = Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap();

        session_manager.expect_execute_query()
            .withf(|query: &str| query == fixture::EXPECTED_QUERY)
            .times(1)
            .returning(move |_| fixture::create_query_result(CqlValue::Text(fixture::EXPECTED_VEHICLE_NAME.to_string())));

        let vehicle_repository = VehicleRepositoryImpl::new(Arc::new(session_manager));

        let vehicle = aw!(vehicle_repository.get_vehicle(user_id, vehicle_id)).unwrap();

        assert_eq!(fixture::EXPECTED_VEHICLE_NAME, vehicle.name);
        assert_eq!(user_id, vehicle.user_id);
        assert_eq!(vehicle_id, vehicle.vehicle_id);
        assert_eq!(Duration::seconds(fixture::EXPECTED_CREATED_AT), vehicle.created_at);
        assert_eq!(fixture::EXPECTED_VEHICLE_TYPE, vehicle.vehicle_type);
        assert_eq!(Duration::seconds(fixture::EXPECTED_RETIRED_AT), vehicle.retired_at.unwrap());
        assert_eq!(fixture::EXPECTED_BRAND, vehicle.brand);
        assert_eq!(fixture::EXPECTED_MODEL, vehicle.model);
        assert_eq!(fixture::EXPECTED_DISTANCE, vehicle.distance);
        assert_eq!(NaiveDate::from_cql(CqlValue::Date(fixture::EXPECTED_OWNER_SINCE)).unwrap(), vehicle.owner_since);
        assert_eq!(NaiveDate::from_cql(CqlValue::Date(fixture::EXPECTED_MANUFACTURING_DATE)).unwrap(), vehicle.manufacturing_date);
        assert_eq!(fixture::EXPECTED_PICTURE, vehicle.picture.unwrap());
    }

    #[test]
    fn given_no_matching_row_when_get_vehicle_then_returns_none() {
        let mut session_manager = MockSessionManagerImpl::new();

        let user_id = Uuid::parse_str(fixture::USER_ID_STR).unwrap();
        let vehicle_id = Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap();

        session_manager.expect_execute_query()
            .withf(|query: &str| query == fixture::EXPECTED_QUERY)
            .times(1)
            .returning(move |_| Ok(QueryResult::default()));

        let vehicle_repository = VehicleRepositoryImpl::new(Arc::new(session_manager));

        let vehicle = aw!(vehicle_repository.get_vehicle(user_id, vehicle_id));

        assert!(vehicle.is_none());
    }

    #[test]
    #[should_panic]
    fn given_error_when_get_vehicle_then_panics() {
        let mut session_manager = MockSessionManagerImpl::new();

        let user_id = Uuid::parse_str(fixture::USER_ID_STR).unwrap();
        let vehicle_id = Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap();

        session_manager.expect_execute_query()
        .withf(|query: &str| query == fixture::EXPECTED_QUERY)
            .times(1)
            .returning(move |_| Err(QueryError::InvalidMessage("error".to_owned())));

        let vehicle_repository = VehicleRepositoryImpl::new(Arc::new(session_manager));

        aw!(vehicle_repository.get_vehicle(user_id, vehicle_id));
    }

    #[test]
    #[should_panic]
    fn given_row_with_unexpected_type_integer_when_get_vehicle_then_panics() {
        let mut session_manager = MockSessionManagerImpl::new();

        let user_id = Uuid::parse_str(fixture::USER_ID_STR).unwrap();
        let vehicle_id = Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap();

        session_manager.expect_execute_query()
        .withf(|query: &str| query == fixture::EXPECTED_QUERY)
            .times(1)
            .returning(move |_| fixture::create_query_result(CqlValue::Int(7)));

        let vehicle_repository = VehicleRepositoryImpl::new(Arc::new(session_manager));

        aw!(vehicle_repository.get_vehicle(user_id, vehicle_id));
    }

    #[test]
    fn when_save_vehicle_then_returns_vehicle() {
        let mut session_manager = MockSessionManagerImpl::new();

        session_manager.expect_execute_query()
            .withf(|query: &str| query == fixture::EXPECTED_SAVE_QUERY)
            .times(1)
            .returning(move |_| fixture::create_query_result(CqlValue::Text(fixture::EXPECTED_VEHICLE_NAME.to_string())));

        let vehicle_repository = VehicleRepositoryImpl::new(Arc::new(session_manager));

        let vehicle = aw!(vehicle_repository.save_vehicle(Vehicle {
            user_id             : Uuid::parse_str(fixture::USER_ID_STR).unwrap(),
            vehicle_id          : Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap(),
            vehicle_type        : fixture::EXPECTED_VEHICLE_TYPE.to_string(),
            name                : fixture::EXPECTED_VEHICLE_NAME.to_string(),
            created_at          : Duration::seconds(fixture::EXPECTED_CREATED_AT),
            retired_at          : Some(Duration::seconds(fixture::EXPECTED_RETIRED_AT)),
            brand               : fixture::EXPECTED_BRAND.to_string(),
            model               : fixture::EXPECTED_MODEL.to_string(),
            distance            : fixture::EXPECTED_DISTANCE,
            owner_since         : NaiveDate::from_num_days_from_ce(15),
            manufacturing_date  : NaiveDate::from_num_days_from_ce(15),
            picture             : Some(fixture::EXPECTED_PICTURE.to_string())
        })).unwrap();

        assert_eq!(fixture::EXPECTED_VEHICLE_NAME, vehicle.name);
    }

    #[test]
    fn given_error_when_save_vehicle_then_returns_none() {
        let mut session_manager = MockSessionManagerImpl::new();

        session_manager.expect_execute_query()
        .withf(|query: &str| query == fixture::EXPECTED_SAVE_QUERY)
            .times(1)
            .returning(move |_| Err(QueryError::InvalidMessage("error".to_owned())));

        let vehicle_repository = VehicleRepositoryImpl::new(Arc::new(session_manager));

        let vehicle = aw!(vehicle_repository.save_vehicle(Vehicle {
            user_id             : Uuid::parse_str(fixture::USER_ID_STR).unwrap(),
            vehicle_id          : Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap(),
            vehicle_type        : fixture::EXPECTED_VEHICLE_TYPE.to_string(),
            name                : fixture::EXPECTED_VEHICLE_NAME.to_string(),
            created_at          : Duration::seconds(fixture::EXPECTED_CREATED_AT),
            retired_at          : Some(Duration::seconds(fixture::EXPECTED_RETIRED_AT)),
            brand               : fixture::EXPECTED_BRAND.to_string(),
            model               : fixture::EXPECTED_MODEL.to_string(),
            distance            : fixture::EXPECTED_DISTANCE,
            owner_since         : NaiveDate::from_num_days_from_ce(15),
            manufacturing_date  : NaiveDate::from_num_days_from_ce(15),
            picture             : Some(fixture::EXPECTED_PICTURE.to_string())
        }));

        assert!(vehicle.is_none());
    }

    mod fixture {
        use super::*;
        use scylla::frame::response::result::Row;
        use scylla::frame::response::result::CqlValue;
        use chrono::Duration;

        pub const USER_ID_STR: &str = "a906615e-2e6a-4edb-9377-5a6b8544791b";
        pub const VEHICLE_ID_STR: &str = "88573010-cf4c-490e-9d29-f8517dc60b90";
        pub const EXPECTED_VEHICLE_NAME: &str = "the vehicle name";
        pub const EXPECTED_CREATED_AT: i64 = 5;
        pub const EXPECTED_VEHICLE_TYPE: &str = "bike";
        pub const EXPECTED_RETIRED_AT: i64 = 10;
        pub const EXPECTED_BRAND: &str = "the brand";
        pub const EXPECTED_MODEL: &str = "the model";
        pub const EXPECTED_DISTANCE: i32 = 500;
        pub const EXPECTED_OWNER_SINCE: u32 = 2147499963;
        pub const EXPECTED_MANUFACTURING_DATE: u32 = 2147499963;
        pub const EXPECTED_PICTURE: &str = "the picture";
        pub const EXPECTED_QUERY: &str = "SELECT name, user_id, vehicle_id, created_at, vehicle_type, retired_at, brand, model, distance, owner_since, manufacturing_date, picture \
            FROM vehicles.vehicle \
            WHERE user_id = a906615e-2e6a-4edb-9377-5a6b8544791b and vehicle_id = 88573010-cf4c-490e-9d29-f8517dc60b90";
        pub const EXPECTED_SAVE_QUERY: &str = "INSERT INTO vehicles.vehicle   (user_id,       \
                                            vehicle_id,    \
                                            vehicle_type,  \
                                            name,          \
                                            created_at,    \
                                            retired_at,    \
                                            brand,         \
                                            model,         \
                                            distance,      \
                                            owner_since,   \
                                            manufacturing_date, \
                                            picture) VALUES (a906615e-2e6a-4edb-9377-5a6b8544791b, 88573010-cf4c-490e-9d29-f8517dc60b90, 'bike', 'the vehicle name', '1970-01-01 00:00:05 UTC', '1970-01-01 00:00:10 UTC', 'the brand', 'the model', 500, '0001-01-15', '0001-01-15', 'the picture')";

        pub fn create_query_result(cql_value: CqlValue) -> Result<QueryResult, QueryError> {
            let cql_values = vec!(
                Some(cql_value),
                Some(CqlValue::Uuid(Uuid::parse_str(USER_ID_STR).unwrap())),
                Some(CqlValue::Uuid(Uuid::parse_str(VEHICLE_ID_STR).unwrap())),
                Some(CqlValue::Timestamp(Duration::seconds(EXPECTED_CREATED_AT))),
                Some(CqlValue::Text(EXPECTED_VEHICLE_TYPE.to_string())),
                Some(CqlValue::Timestamp(Duration::seconds(EXPECTED_RETIRED_AT))),
                Some(CqlValue::Text(EXPECTED_BRAND.to_string())),
                Some(CqlValue::Text(EXPECTED_MODEL.to_string())),
                Some(CqlValue::Int(EXPECTED_DISTANCE)),
                Some(CqlValue::Date(EXPECTED_OWNER_SINCE)),
                Some(CqlValue::Date(EXPECTED_MANUFACTURING_DATE)),
                Some(CqlValue::Text(EXPECTED_PICTURE.to_string())));
            let row = Row {
                columns: cql_values
            };
            let empty_vec = vec!();
            let rows = Some(vec!(row));

            Ok(QueryResult {
                rows,
                warnings: empty_vec,
                tracing_id: None,
                paging_state: None
            })
        }
    }
}