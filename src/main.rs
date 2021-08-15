#[macro_use] extern crate rocket;

mod domain { pub mod vehicle; }
mod dto { pub mod book; }
mod dao { pub mod session_manager; }
mod service { pub mod vehicle_service; }
mod repository { pub mod vehicle_repository; }
mod controller {
    pub mod controllers;
    pub mod catchers;
}

use std::sync::Arc;
use crate::dao::session_manager::SessionManager;
use crate::repository::vehicle_repository::VehicleRepository;
use crate::service::vehicle_service::VehicleService;
use crate::controller::controllers;
use crate::controller::catchers;

const NODE: &str = "nuckito:9042";

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let session_manager = SessionManager::new(NODE).await;
    let vehicle_repository = VehicleRepository::new(Arc::new(session_manager));
    let vehicle_service = VehicleService::new(vehicle_repository);

    rocket::build()
      .register("/", catchers![catchers::internal_error, catchers::not_found])
      .mount("/api", routes![controllers::get_vehicle, controllers::hello, controllers::new_book])
      .manage(vehicle_service)
      .launch()
      .await
}
