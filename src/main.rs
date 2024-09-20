use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use log::{info, error};
use env_logger;

#[derive(Serialize, Deserialize)]
struct TripData {
    tpep_pickup_datetime: DateTime<Utc>,
    tpep_dropoff_datetime: DateTime<Utc>,
    pu_location_id: i32,
    do_location_id: i32,
    trip_distance: f64,
}

async fn get_trip_data() -> impl Responder {
    info!("Handling GET request for trip data");
    
    let trip = TripData {
        tpep_pickup_datetime: "2024-01-01T00:57:55Z".parse().unwrap(),
        tpep_dropoff_datetime: "2024-01-01T01:17:43Z".parse().unwrap(),
        pu_location_id: 186,
        do_location_id: 79,
        trip_distance: 1.72,
    };
    HttpResponse::Ok().json(trip)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Starting server");
    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .route("/trip", web::get().to(get_trip_data))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await.map_err(|e| {
        error!("Server error: {}", e);
        e
    })
}