use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::State;
use axum::Form;
use http::StatusCode;
use serde::Deserialize;

use crate::app_state::AppState;

#[derive(Deserialize)]
pub struct SearchFormData {
    search_query: String,
}

struct LocationDetails {
    name: String,
    country: String,
    lat: f64,
    lon: f64,
}

#[derive(Template)]
#[template(path = "locations_list.html")]
struct LocationListTemplate {
    locations: Vec<LocationDetails>,
}

pub async fn locations(
    State(app_state): State<AppState>,
    Form(search_form_data): Form<SearchFormData>,
) -> impl IntoResponse {
    match app_state
        .open_weather_map_api
        .get_geocoding(search_form_data.search_query)
        .await
    {
        Ok(geocoding_response) => LocationListTemplate {
            locations: geocoding_response
                .iter()
                .map(|location| LocationDetails {
                    name: location.name.clone(),
                    country: location.country.clone(),
                    lat: location.lat,
                    lon: location.lon,
                })
                .collect(),
        }
        .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error fetching the weather {error}"),
        )
            .into_response(),
    }
}
