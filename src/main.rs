use std::env;

use axum::{routing::get, Router};
use flight_contract::{FlightContracts, RequestError};
use tokio::net::TcpListener;
use serde::{Serialize, Deserialize};

mod flight_contract;
mod pages;
mod database;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Environment {
    endpoint: String,
    api_key: String,
    redis_url: String,
}
#[tokio::main]
async fn main() {
    // Checks environment variables once on startup
    let env = Environment {
        endpoint: env::var("ENDPOINT").expect("ENDPOINT must be set!"),
        api_key: env::var("API_KEY").expect("API_KEY must be set!"),
        redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set!"),
    };

    // Sets up REDIS connection
    let pool = database::init_conn(env.redis_url.clone()).await;

    // Checks health on startup, and if environment variables are available
    // Sets up web server
    let app = Router::new()
        .route(
            "/flights/{callsign}/{local_date}", 
            get(pages::flight_details).with_state((env.clone(), pool))
        )
        .route(
            "/health", 
            get(pages::health_check).with_state(env.clone())
        );


    // Webserver options
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let listener_address = listener.local_addr().unwrap();
    println!("Listening on {listener_address}");
    axum::serve(listener, app).await.unwrap();
}