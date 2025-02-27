use std::env;

use axum::{routing::get, Router};
use flight_contract::{FlightContracts, RequestError};
use log::{error, info};
use tokio::net::TcpListener;

mod database;
mod flight_contract;
mod pages;

#[derive(Debug, Clone, PartialEq)]
struct Environment {
    endpoint: String,
    api_key: String,
    redis_host: String,
    redis_port: String,
}

// Uses env_logger to log error instead of just panicking. Is this really better?
fn env_err(var: String) -> String {
    match env::var(&var) {
        Ok(val) => {
            match var.as_str() { // Hide API_KEY
                "API_KEY" => info!("API_KEY: ..."),
                _ => info!("{var}: {val}")
            }
            val
        },
        Err(_) => {
            error!("{var}: Not set");
            panic!()
        }
    }
}

#[tokio::main]
async fn main() {
    // Start env_logger
    env_logger::init();

    loop {
        // Checks environment variables once on startup
        // Won't check again until restart
        let env = Environment {
            endpoint: env_err("ENDPOINT".to_string()),
            api_key: env_err("API_KEY".to_string()),
            redis_host: env_err("REDIS_HOST".to_string()),
            redis_port: env_err("REDIS_PORT".to_string()),
        };

        // Sets up REDIS connection
        let pool = database::init_conn(format!(
            "redis://{}:{}",
            env.redis_host.clone(),
            env.redis_port.clone()
        ))
        .await;

        // Checks health on startup, and if environment variables are available
        // Sets up web server
        let app = Router::new()
            .route(
                "/flights/{callsign}/{local_date}",
                get(pages::flight_details).with_state((env.clone(), pool)),
            )
            .route("/health", get(pages::health_check).with_state(env.clone()));

        // Webserver options
        // Not designed for direct connection: Use with reverse proxy instead...
        let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
        let listener_address = listener.local_addr().unwrap();
        info!("Listening on {listener_address}");
        axum::serve(listener, app).await.unwrap();
    }
}
