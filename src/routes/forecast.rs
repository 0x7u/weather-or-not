use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::State;
use axum::Form;
use chrono::DateTime;
use http::StatusCode;
use serde::Deserialize;

use crate::app_state::AppState;

#[derive(Deserialize)]
pub struct ForecastFormData {
    lat: f64,
    lon: f64,
}

struct Forecast {
    datetime: String,
    status: String,
    temperature: f64,
}

#[derive(Template)]
#[template(path = "forecast.html")]
struct ForecastTemplate {
    forecasts: Vec<Forecast>,
}

pub async fn forecast(
    State(AppState {
        open_weather_map_api,
    }): State<AppState>,
    Form(ForecastFormData { lat, lon }): Form<ForecastFormData>,
) -> impl IntoResponse {
    match open_weather_map_api
        .get_five_days_forecast((lat, lon))
        .await
    {
        Ok(five_days_forecast_response) => {
            let forecasts = five_days_forecast_response
                .list
                .iter()
                .map(|forecast| Forecast {
                    datetime: DateTime::from_timestamp(forecast.dt, 0)
                        .unwrap()
                        .format("%a, %I %p")
                        .to_string(),
                    status: forecast.weather.first().unwrap().main.clone(),
                    temperature: forecast.main.temp,
                })
                .collect();

            ForecastTemplate { forecasts }
        }
        .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error fetching the forecast {error}"),
        )
            .into_response(),
    }
}
