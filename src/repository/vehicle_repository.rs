use scylla::{SessionBuilder, Session};
use scylla::IntoTypedRows;
use scylla::transport::retry_policy::DefaultRetryPolicy;
use scylla::query::Query;

use tokio_retry::Retry;
use tokio_retry::strategy::{ExponentialBackoff, jitter};

use rocket::serde::uuid::Uuid;

use crate::domain::vehicle::VehicleName;

pub struct VehicleRepository {
    session: Session
}

impl VehicleRepository {
    pub async fn new(node: &str) -> VehicleRepository {
        let session: Session = SessionBuilder::new()
            .known_node(node)
            .retry_policy(Box::new(DefaultRetryPolicy::new()))
            .build()
            .await
            .expect(&format!("Failed to connect {}", node));

        VehicleRepository {
            session
        }
    }

    pub async fn get_vehicle_name(&self, user_id: &Uuid, &vehicle_id: &Uuid) -> Option<VehicleName> {
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
                for row in rows.into_typed::<VehicleName>() {
                    let vehicle_name: VehicleName = row.expect("Failed to extract VehicleName from Row");
                    return Some(vehicle_name);
                }
            };

        None
    }
}
