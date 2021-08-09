#[macro_use] extern crate rocket;

mod domain { pub mod vehicle; }
mod dto { pub mod book; }
mod service { pub mod vehicle_service; }
mod repository { pub mod vehicle_repository; }
mod controller {
    pub mod controllers;
    pub mod catchers;
}

use crate::repository::vehicle_repository::VehicleRepository;
use crate::service::vehicle_service::VehicleService;
use crate::controller::controllers;
use crate::controller::catchers;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let vehicle_repository = VehicleRepository::new("nuckito:9042").await;
    let vehicle_service = VehicleService::new(vehicle_repository).await;

    rocket::build()
      .register("/", catchers![catchers::internal_error, catchers::not_found])
      .mount("/api", routes![controllers::get_vehicle, controllers::hello, controllers::new_book])
      .manage(vehicle_service)
      .launch()
      .await
}
