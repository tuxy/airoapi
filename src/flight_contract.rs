use serde::{Deserialize, Serialize};

// If error was to occur, an error would be returned to client
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestError {
    pub message: String,
}

// Flight Contract schema, including everything with optional values
pub type FlightContracts = Vec<FlightContract>;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlightContract {
    pub great_circle_distance: GreatCircleDistance,
    pub departure: Flight,
    pub arrival: Flight,
    pub last_updated_utc: String,
    pub number: String,
    pub call_sign: Option<String>,
    pub status: String,
    pub codeshare_status: String,
    pub is_cargo: bool,
    pub aircraft: Aircraft,
    pub airline: Option<Airline>,
    pub location: Option<Location>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GreatCircleDistance {
    pub meter: f32,
    pub km: f32,
    pub mile: f32,
    pub nm: f32,
    pub feet: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flight {
    pub airport: Airport,
    pub scheduled_time: Option<Time>,
    pub revised_time: Option<Time>,
    pub predicted_time: Option<Time>,
    pub runway_time: Option<Time>,
    pub terminal: Option<String>,
    pub check_in_desk: Option<String>,
    pub gate: Option<String>,
    pub baggage_belt: Option<String>,
    pub runway: Option<String>,
    pub quality: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Airport {
    pub icao: Option<String>,
    pub iata: Option<String>,
    pub local_code: Option<String>,
    pub name: String,
    pub short_name: Option<String>,
    pub municipality_name: Option<String>,
    pub location: Option<Location>,
    pub country_code: Option<String>,
    pub time_zone: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub lat: f32,
    pub lon: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Time {
    pub utc: String,
    pub local: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aircraft {
    pub reg: Option<String>,
    pub mode_s: Option<String>,
    pub model: Option<String>,
    pub image: Option<Image>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub url: String,
    pub web_url: Option<String>,
    pub author: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub license: String,
    pub html_attributions: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Airline {
    pub name: String,
    pub iata: Option<String>,
    pub icao: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PressureAltitude {
    pub meter: f32,
    pub km: f32,
    pub mile: f32,
    pub nm: f32,
    pub feet: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Altitude {
    pub meter: f32,
    pub km: f32,
    pub mile: f32,
    pub nm: f32,
    pub feet: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pressure {
    pub h_pa: f32,
    pub in_hg: f32,
    pub mm_hg: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroundSpeed {
    pub kt: f32,
    pub km_per_hour: f32,
    pub mi_per_hour: f32,
    pub meter_per_second: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrueTrack {
    pub deg: f32,
    pub rad: f64,
}
