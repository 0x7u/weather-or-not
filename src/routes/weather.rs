use askama::Template;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use tokio::join;

use crate::app_state::AppState;

#[derive(Deserialize)]
pub struct WeatherQueryData {
    lat: f64,
    lon: f64,
}

#[derive(Template)]
#[template(path = "weather.html")]
struct WeatherTemplate<'a> {
    status: Option<&'a str>,
    description: Option<&'a str>,
    temperature: f64,
    feels_like: f64,
    humidity: i64,
    wind_speed: f64,
    aqi: Option<i64>,
    pm2_5: Option<f64>,
}

pub async fn weather(
    State(AppState {
        open_weather_map_api,
    }): State<AppState>,
    Query(WeatherQueryData { lat, lon }): Query<WeatherQueryData>,
) -> impl IntoResponse {
    let coord = (lat, lon);

    match join!(
        open_weather_map_api.get_weather(coord),
        open_weather_map_api.get_air_quality(coord),
    ) {
        (Ok(weather_response), Ok(air_quality_response)) => WeatherTemplate {
            status: weather_response
                .weather
                .first()
                .map(|weather| weather.main.as_str()),
            description: weather_response
                .weather
                .first()
                .map(|weather| weather.description.as_str()),
            temperature: weather_response.main.temp,
            feels_like: weather_response.main.feels_like,
            humidity: weather_response.main.humidity,
            wind_speed: weather_response.wind.speed,
            aqi: air_quality_response
                .list
                .first()
                .map(|weather_response| weather_response.main.aqi),
            pm2_5: air_quality_response
                .list
                .first()
                .map(|weather_response| weather_response.components.pm2_5),
        }
        .into_response(),
        (Err(error), _) | (_, Err(error)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error fetching the weather {error}"),
        )
            .into_response(),
    }
}
