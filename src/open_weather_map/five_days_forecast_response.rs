use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiveDaysForecastResponse {
    pub cod: String,
    pub message: i64,
    pub cnt: i64,
    pub list: Vec<List>,
    pub city: City,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct List {
    pub dt: i64,
    pub main: Main,
    pub weather: Vec<Weather>,
    pub clouds: Clouds,
    pub wind: Wind,
    pub visibility: i64,
    pub pop: f64,
    pub sys: Sys,
    #[serde(rename = "dt_txt")]
    pub dt_txt: String,
    pub rain: Option<Rain>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Main {
    pub temp: f64,
    #[serde(rename = "feels_like")]
    pub feels_like: f64,
    #[serde(rename = "temp_min")]
    pub temp_min: f64,
    #[serde(rename = "temp_max")]
    pub temp_max: f64,
    pub pressure: i64,
    #[serde(rename = "sea_level")]
    pub sea_level: i64,
    #[serde(rename = "grnd_level")]
    pub grnd_level: i64,
    pub humidity: i64,
    #[serde(rename = "temp_kf")]
    pub temp_kf: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clouds {
    pub all: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wind {
    pub speed: f64,
    pub deg: i64,
    pub gust: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sys {
    pub pod: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rain {
    #[serde(rename = "3h")]
    pub n3h: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct City {
    pub id: i64,
    pub name: String,
    pub coord: Coord,
    pub country: String,
    pub population: i64,
    pub timezone: i64,
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coord {
    pub lat: f64,
    pub lon: f64,
}
