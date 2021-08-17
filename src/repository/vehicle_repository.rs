use std::sync::Arc;
use scylla::IntoTypedRows;

use rocket::serde::uuid::Uuid;

use crate::dao::session_manager::Queriable;
use crate::domain::vehicle::Vehicle;

#[async_trait]
pub trait VehicleManager {
    async fn get_vehicle_name(&self, user_id: Uuid, vehicle_id: Uuid) -> Option<Vehicle>;
}

pub struct VehicleRepository {
    queriable: Arc<dyn Queriable + Sync + Send + 'static>,
}

impl VehicleRepository {
    pub fn new(queriable: Arc<dyn Queriable + Sync + Send + 'static>) -> VehicleRepository {
        VehicleRepository {
            queriable
        }
    }
}

#[async_trait]
impl VehicleManager for VehicleRepository {
    async fn get_vehicle_name(&self, user_id: Uuid, vehicle_id: Uuid) -> Option<Vehicle> {
        let query = format!("SELECT name FROM vehicles.vehicle WHERE user_id = {} and vehicle_id = {}", user_id, vehicle_id);

        let result = self.queriable.execute_query(&query).await;

        if let Some(rows) = result
            .expect("Failed to execute query") // TODO return Result instead of failing here
            .rows {
                for row in rows.into_typed::<Vehicle>() {
                    let vehicle_name: Vehicle = row.expect("Failed to extract VehicleName from Row");
                    return Some(vehicle_name);
                }
            };

        None
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use scylla::QueryResult;
    use scylla::transport::errors::QueryError;
    use scylla::frame::response::result::CqlValue;

    use mockall::{mock, predicate::*};

    macro_rules! aw {
        ($e: expr) => {
            tokio_test::block_on($e)
        };
    }

    mock! {
        SessionManager {}

        #[async_trait]
        impl Queriable for SessionManager {
            async fn execute_query(&self, query_statement: &str) -> Result<QueryResult, QueryError>;
        }
    }

    #[test]
    fn when_get_vehicle_name_then_returns_vehicle_name() {
        let mut session_manager = MockSessionManager::new();

        let user_id = Uuid::parse_str(fixture::USER_ID_STR).unwrap();
        let vehicle_id = Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap();

        session_manager.expect_execute_query()
            .withf(|query: &str| query == fixture::EXPECTED_QUERY)
            .times(1)
            .returning(move |_| fixture::create_query_result(CqlValue::Text(fixture::EXPECTED_VEHICLE_NAME.to_string())));

        let vehicle_repository = VehicleRepository::new(Arc::new(session_manager));

        let vehicle_name = aw!(vehicle_repository.get_vehicle_name(user_id, vehicle_id)).unwrap();

        assert_eq!(fixture::EXPECTED_VEHICLE_NAME, vehicle_name.name);
    }

    #[test]
    fn given_no_matching_row_when_get_vehicle_name_then_returns_none() {
        let mut session_manager = MockSessionManager::new();

        let user_id = Uuid::parse_str(fixture::USER_ID_STR).unwrap();
        let vehicle_id = Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap();

        session_manager.expect_execute_query()
            .withf(|query: &str| query == fixture::EXPECTED_QUERY)
            .times(1)
            .returning(move |_| Ok(QueryResult::default()));

        let vehicle_repository = VehicleRepository::new(Arc::new(session_manager));

        let vehicle_name = aw!(vehicle_repository.get_vehicle_name(user_id, vehicle_id));

        assert!(vehicle_name.is_none());
    }

    #[test]
    #[should_panic]
    fn given_error_when_get_vehicle_name_then_panics() {
        let mut session_manager = MockSessionManager::new();

        let user_id = Uuid::parse_str(fixture::USER_ID_STR).unwrap();
        let vehicle_id = Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap();

        session_manager.expect_execute_query()
        .withf(|query: &str| query == fixture::EXPECTED_QUERY)
            .times(1)
            .returning(move |_| Err(QueryError::InvalidMessage("error".to_owned())));

        let vehicle_repository = VehicleRepository::new(Arc::new(session_manager));

        aw!(vehicle_repository.get_vehicle_name(user_id, vehicle_id));
    }

    #[test]
    #[should_panic]
    fn given_row_with_unexpected_type_integer_when_get_vehicle_name_then_panics() {
        let mut session_manager = MockSessionManager::new();

        let user_id = Uuid::parse_str(fixture::USER_ID_STR).unwrap();
        let vehicle_id = Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap();

        session_manager.expect_execute_query()
        .withf(|query: &str| query == fixture::EXPECTED_QUERY)
            .times(1)
            .returning(move |_| fixture::create_query_result(CqlValue::Int(7)));

        let vehicle_repository = VehicleRepository::new(Arc::new(session_manager));

        aw!(vehicle_repository.get_vehicle_name(user_id, vehicle_id));
    }

    mod fixture {
        use super::*;
        use scylla::frame::response::result::Row;
        use scylla::frame::response::result::CqlValue;

        pub const USER_ID_STR: &str = "a906615e-2e6a-4edb-9377-5a6b8544791b";
        pub const VEHICLE_ID_STR: &str = "88573010-cf4c-490e-9d29-f8517dc60b90";
        pub const EXPECTED_VEHICLE_NAME: &str = "the vehicle name";
        pub const EXPECTED_QUERY: &str = "SELECT name FROM vehicles.vehicle WHERE user_id = a906615e-2e6a-4edb-9377-5a6b8544791b and vehicle_id = 88573010-cf4c-490e-9d29-f8517dc60b90";

        pub fn create_query_result(cql_value: CqlValue) -> Result<QueryResult, QueryError> {
            let cql_values = vec!(Some(cql_value));
            let row = Row {
                columns: cql_values
            };
            let empty_vec = vec!();
            let rows = Some(vec!(row));

            Ok(QueryResult {
                rows: rows,
                warnings: empty_vec,
                tracing_id: None,
                paging_state: None
            })
        }
    }
}