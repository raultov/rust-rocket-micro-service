#[macro_use] extern crate rocket;

mod domain {
    pub mod vehicle;
    pub mod vehicle_type;
}
mod dto {
    pub mod book;
    pub mod vehicle_dto;
}
mod dao { pub mod session_manager; }
mod service { pub mod vehicle_service; }
mod mapper { pub mod vehicle_mapper; } 
mod repository { pub mod vehicle_repository; }
mod controller {
    pub mod controllers;
    pub mod catchers;
}

use std::sync::Arc;
use std::env;

use crate::dao::session_manager::SessionManagerImpl;
use crate::repository::vehicle_repository::VehicleRepositoryImpl;
use crate::service::vehicle_service::VehicleService;
use crate::controller::controllers;
use crate::controller::catchers;

const CASSANDRA_NODE: &str = "localhost:9042";

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let cassandra_node = env::var("CASSANDRA_NODE").unwrap_or_else(|_| CASSANDRA_NODE.to_string());

    let session_manager = SessionManagerImpl::new(&cassandra_node).await;
    let vehicle_repository = VehicleRepositoryImpl::new(Arc::new(session_manager));
    let vehicle_service = VehicleService::new(Arc::new(vehicle_repository));

    rocket(Arc::new(vehicle_service))
      .launch()
      .await
}

fn rocket(vehicle_service: Arc<VehicleService>) -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .register("/", catchers![catchers::internal_error, catchers::not_found])
        .mount("/api", routes![controllers::get_vehicle, controllers::hello, controllers::new_book, controllers::new_vehicle])
        .manage(vehicle_service)
}
