use serde::{Deserialize, Serialize};

pub type GeocodingResponse = Vec<Location>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub country: String,
}
