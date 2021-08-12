use cfg_if::cfg_if;

use scylla::IntoTypedRows;
use scylla::transport::retry_policy::DefaultRetryPolicy;
use scylla::query::Query;

use tokio_retry::Retry;
use tokio_retry::strategy::{ExponentialBackoff, jitter};

use rocket::serde::uuid::Uuid;

use crate::domain::vehicle::Vehicle;

pub struct VehicleRepository {
    session: Session
}

impl VehicleRepository {
    pub async fn new(node: &str) -> VehicleRepository {

        let session = VehicleRepository::create_session(node).await;

        VehicleRepository {
            session
        }
    }

    pub async fn get_vehicle_name(&self, user_id: Uuid, vehicle_id: Uuid) -> Option<Vehicle> {
        let retry_strategy = ExponentialBackoff::from_millis(10)
            .map(jitter) // add jitter to delays
            .take(3);    // limit to 3 retries

        let result = Retry::spawn(retry_strategy, || {
            let mut get_vehicle_query: Query = Query::new("SELECT name FROM vehicles.vehicle WHERE user_id = ? and vehicle_id = ?".to_string());
            get_vehicle_query.set_retry_policy(Box::new(DefaultRetryPolicy::new()));
            self.session.query(get_vehicle_query, (user_id, vehicle_id))
        }).await;

        if let Some(rows) = result
            .expect("Failed to execute query")
            .rows {
                for row in rows.into_typed::<Vehicle>() {
                    let vehicle_name: Vehicle = row.expect("Failed to extract VehicleName from Row");
                    return Some(vehicle_name);
                }
            };

        None
    }

    async fn create_session(_node: &str) -> Session {
        cfg_if! {
            if #[cfg(test)] {
                tests::MockSession::new()
            } else {
                SessionBuilder::new()
                    .known_node(_node)
                    .build()
                    .await
                    .expect(&format!("Failed to connect {}", _node))
            }
        }
    }
}

cfg_if! {
    if #[cfg(test)] {
        use tests::MockSession as Session;
        use tests::Queryable;

        macro_rules! aw {
            ($e: expr) => {
                tokio_test::block_on($e)
            };
        }
    } else {
        use scylla::Session;
        use scylla::SessionBuilder;
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use scylla::QueryResult;
    use scylla::transport::errors::QueryError;
    use scylla::query::Query;

    use mockall::{automock, mock, predicate::*};

    #[automock]
    #[async_trait]
    pub trait Queryable {
        async fn query(&self, query: Query, values: (Uuid, Uuid)) -> Result<QueryResult, QueryError>;
    }

    mock! {
        pub Session {}

        #[async_trait]
        impl Queryable for Session {
            pub async fn query(&self, query: Query, values: (Uuid, Uuid)) -> Result<QueryResult, QueryError>;
        }
    }

    #[test]
    fn given_user_id_and_vehicle_id_when_get_vehicle_name_then_returns_vehicle_name() {
        let mut vehicle_repository = aw!(VehicleRepository::new("node"));

        let _get_vehicle_query: Query = Query::new(fixture::QUERY_STR.to_string());
        let user_id = Uuid::parse_str(fixture::USER_ID_STR).unwrap();
        let vehicle_id = Uuid::parse_str(fixture::VEHICLE_ID_STR).unwrap();

        vehicle_repository.session.expect_query().returning(move |_get_vehicle_query, (_user_id, _vehicle_id)| fixture::forge_query_result());

        let vehicle_name = aw!(vehicle_repository.get_vehicle_name(user_id, vehicle_id)).expect("shit happens");

        assert_eq!(fixture::EXPECTED_VEHICLE_NAME, vehicle_name.name);
    }

    mod fixture {
        use super::*;

        pub const QUERY_STR: &str = "SELECT";
        pub const USER_ID_STR: &str = "a906615e-2e6a-4edb-9377-5a6b8544791b";
        pub const VEHICLE_ID_STR: &str = "88573010-cf4c-490e-9d29-f8517dc60b90";
        pub const EXPECTED_VEHICLE_NAME: &str = "the vehicle name";

        pub fn forge_query_result() -> Result<QueryResult, QueryError> {
            let cql_values = vec!(Some(scylla::frame::response::result::CqlValue::Text(EXPECTED_VEHICLE_NAME.to_string())));
            let row = scylla::frame::response::result::Row {
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
