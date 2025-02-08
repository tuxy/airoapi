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
    redis_host: String,
    redis_port: String
}
#[tokio::main]
async fn main() {
    loop { // Janky way to force a restart
        // Checks environment variables once on startup
        let env = Environment {
            endpoint: env::var("ENDPOINT").expect("ENDPOINT must be set!"),
            api_key: env::var("API_KEY").expect("API_KEY must be set!"),
            redis_host: env::var("REDIS_HOST").expect("REDIS_HOST must be set!"),
            redis_port: env::var("REDIS_PORT").expect("REDIS_PORT must be set!"),
        };

        // Sets up REDIS connection
        let pool = database::init_conn(
            format!("redis://{}:{}", env.redis_host.clone(), env.redis_port.clone())
        ).await;

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
        let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
        let listener_address = listener.local_addr().unwrap();
        println!("Listening on {listener_address}");
        axum::serve(listener, app).await.unwrap();
    }
}