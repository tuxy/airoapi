use crate::{
    flight_contract::FlightContract,
    {Environment, FlightContracts, RequestError},
};
use axum::{
    extract::{Path, State},
    Json,
};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use log::warn;
use redis::AsyncCommands;
use reqwest::{header, redirect, StatusCode};
use serde::{Deserialize, Serialize};

// This represents the health of the aerodatabox api
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub service: String,
    pub status: String,
}

// Checks the health of the API server, and whether it can be reached
pub async fn health_check(State(env): State<Environment>) -> String {
    let url = env.endpoint + "/health/services/feeds/FlightSchedules";

    let mut headers: header::HeaderMap = header::HeaderMap::new();
    headers.insert("x-magicapi-key", env.api_key.parse().unwrap());

    let client = reqwest::Client::builder()
        .redirect(redirect::Policy::none())
        .build()
        .unwrap();

    // Sees if the client errors out
    let status = match client.get(url).headers(headers).send().await {
        Ok(val) => match val.json::<ServiceStatus>().await {
            Ok(json) => json.status,
            Err(_) => String::from("API Down"),
        },
        Err(_) => String::from("Network Issue"),
    };

    warn!("API currently unreachable: {status}");
    status
}

// Might return a RequestError
pub async fn flight_details(
    State((env, pool)): State<(Environment, Pool<RedisConnectionManager>)>,
    Path((number, date)): Path<(String, String)>,
) -> Result<Json<FlightContracts>, Json<RequestError>> {
    match is_date(&date).await {
        true => {} // If date is correct, continue
        false => {
            return Err(Json(RequestError {
                // Network errors, etc
                message: String::from("Invalid date provided"),
            }));
        }
    }

    let mut conn = pool.get().await.unwrap();
    let cached_flight = conn
        .get::<String, Option<String>>(format!("{number}*{date}"))
        .await
        .unwrap();

    match cached_flight {
        Some(flight) => {
            let json: FlightContract = serde_json::from_str(&flight).unwrap();
            Ok(Json(vec![json])) // To ensure compatability with client, the result is put into a vec
        }
        None => {
            let request_string = format!("{}/flights/number/{}/{}", env.endpoint, number, date);

            let mut headers = header::HeaderMap::new();
            headers.insert("x-magicapi-key", env.api_key.parse().unwrap());

            let params = [("withAircraftImage", "true")];

            let url = reqwest::Url::parse_with_params(&request_string, &params).unwrap();

            let client = reqwest::Client::builder()
                .redirect(redirect::Policy::none())
                .build()
                .unwrap();

            // Returns either a successful response, or a RequestError JSON
            match client.get(url).headers(headers).send().await {
                Ok(val) => {
                    // Server responds with no content
                    if val.status() == StatusCode::NO_CONTENT {
                        return Err(
                            Json(
                                RequestError {
                                    message: String::from("No flight found")
                                }
                            )
                        )
                    }

                    // Checks whether the server response can be parsed
                    match val.json::<FlightContracts>().await {
                        Ok(json) => {
                            // Caches result of API to redis
                            conn.set_ex::<&str, String, Option<String>>(
                                &format!("{number}*{date}"),
                                // Caches only the start, to ensure compatability
                                serde_json::to_string(&json[0]).unwrap(),
                                86400,
                            )
                            .await
                            .unwrap();

                            Ok(Json(json))
                        }
                        Err(_) => Err(Json(RequestError {
                            // JSON Parsing error, invalid
                            message: String::from("Could not parse server response"),
                        })),
                    }
                }
                Err(_) => Err(Json(RequestError {
                    // Network errors, etc
                    message: String::from("Could not contact server"),
                })),
            }
        }
    }
}

// Checks if date is somewhat valid
async fn is_date(date: &str) -> bool {
    let split_date = date.split("-").collect::<Vec<&str>>();

    if split_date.len() != 3 {
        return false;
    }
    true
}
