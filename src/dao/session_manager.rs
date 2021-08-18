use scylla::transport::errors::QueryError;
use scylla::QueryResult;
use scylla::query::Query;

use cfg_if::cfg_if;

use async_trait::async_trait;

use tokio_retry::Retry;
use tokio_retry::strategy::{ExponentialBackoff, jitter};

#[async_trait]
pub trait SessionManager {
    async fn execute_query(&self, query_statement: &str) -> Result<QueryResult, QueryError>;
}

pub struct SessionManagerImpl {
    session: Session
}

impl SessionManagerImpl {
    pub async fn new(_node: &str) -> SessionManagerImpl {
        cfg_if! {
            if #[cfg(test)] {
                let session = tests::MockSession::new();
            } else {
                let session = SessionBuilder::new()
                    .known_node(_node)
                    .build()
                    .await
                    .expect(&format!("Failed to connect {}", _node));
            }
        }

        SessionManagerImpl {
            session
        }
    }
}

#[async_trait]
impl SessionManager for SessionManagerImpl {
    async fn execute_query(&self, query_statement: &str) -> Result<QueryResult, QueryError> {
        let retry_strategy = ExponentialBackoff::from_millis(10)
            .map(jitter) // add jitter to delays
            .take(3);    // limit to 3 retries

        let result = Retry::spawn(retry_strategy, || {
            let query: Query = Query::new(query_statement.to_owned());
            self.session.query(query, ())
        }).await;

        result
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
    use scylla::frame::response::result::CqlValue;

    use mockall::{automock, mock};

    #[automock]
    #[async_trait]
    pub trait Queryable {
        async fn query(&self, query: Query, values: ()) -> Result<QueryResult, QueryError>;
    }

    mock! {
        pub Session {}

        #[async_trait]
        impl Queryable for Session {
            pub async fn query(&self, query: Query, values: ()) -> Result<QueryResult, QueryError>;
        }
    }

    fn get_type_of<T>(_: &T) -> &'static str {
        std::any::type_name::<T>()
    }

    #[test]
    fn when_new_then_returns_session_manager() {
        let session_manager = aw!(SessionManagerImpl::new("node"));

        assert_eq!(get_type_of(&session_manager), "rust_rocket_micro_service::dao::session_manager::SessionManagerImpl");
    }

    #[test]
    fn when_execute_query_then_returns_query_result_ok() {
        let mut session_manager = aw!(SessionManagerImpl::new("node"));

        session_manager.session.expect_query()
            .withf(|query: &Query, _| query.get_contents() == fixture::QUERY_STR.to_string())
            .withf(|_, tuple: &()| tuple == &())
            .times(1)
            .returning(move |_, _| fixture::forge_query_result());

        let result = aw!(session_manager.execute_query(fixture::QUERY_STR));

        if let Some(rows) = result
            .unwrap()
            .rows {
                let cql_value: &CqlValue = rows.first().unwrap().columns.first().unwrap().as_ref().unwrap();
                assert_eq!(*cql_value.as_text().unwrap(), fixture::SOMETHING.to_owned());
            }
        else {
            panic!("No row found");
        }
    }

    #[test]
    fn given_no_matching_row_when_execute_query_then_returns_query_result_ok() {
        let mut session_manager = aw!(SessionManagerImpl::new("node"));

        session_manager.session.expect_query()
            .withf(|query: &Query, _| query.get_contents() == fixture::QUERY_STR.to_string())
            .withf(|_, tuple: &()| tuple == &())
            .times(1)
            .returning(move |_, _| Ok(QueryResult::default()));

        let result = aw!(session_manager.execute_query(fixture::QUERY_STR));

        if let Some(_) = result
            .unwrap()
            .rows {
                panic!("Rows found");
            };
    }

    #[test]
    fn given_error_when_execute_query_then_retries_up_to_4_times_then_returns_query_error() {
        let mut session_manager = aw!(SessionManagerImpl::new("node"));

        session_manager.session.expect_query()
            .withf(|query: &Query, _| query.get_contents() == fixture::QUERY_STR.to_string())
            .withf(|_, tuple: &()| tuple == &())
            .times(4)
            .returning(move |_, _| Err(QueryError::InvalidMessage("error".to_owned())));

        let result = aw!(session_manager.execute_query(fixture::QUERY_STR));

        assert!(result.is_err());
    }

    #[test]
    fn given_single_error_when_execute_query_then_retries_and_returns_query_result_ok() {
        let mut session_manager = aw!(SessionManagerImpl::new("node"));

        session_manager.session.expect_query()
            .withf(|query: &Query, _| query.get_contents() == fixture::QUERY_STR.to_string())
            .withf(|_, tuple: &()| tuple == &())
            .times(1)
            .returning(move |_, _| Err(QueryError::InvalidMessage("error".to_owned())));

        session_manager.session.expect_query()
            .withf(|query: &Query, _| query.get_contents() == fixture::QUERY_STR.to_string())
            .withf(|_, tuple: &()| tuple == &())
            .times(1)
            .returning(move |_, _| fixture::forge_query_result());

            let result = aw!(session_manager.execute_query(fixture::QUERY_STR));

            if let Some(rows) = result
                .unwrap()
                .rows {
                    let cql_value: &CqlValue = rows.first().unwrap().columns.first().unwrap().as_ref().unwrap();
                    assert_eq!(*cql_value.as_text().unwrap(), fixture::SOMETHING.to_owned());
                }
            else {
                panic!("No row found");
            }
    }

    mod fixture {
        use super::*;

        pub const QUERY_STR: &str = "SELECT something FROM anywhere";
        pub const SOMETHING: &str = "something";

        pub fn forge_query_result() -> Result<QueryResult, QueryError> {
            let cql_values = vec!(Some(scylla::frame::response::result::CqlValue::Text(SOMETHING.to_owned())));
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
