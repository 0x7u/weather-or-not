use axum::{
    routing::{get, post},
    Router,
};
use shuttle_secrets::SecretStore;

use crate::app_state::AppState;
use crate::open_weather_map::open_weather_map_api::OpenWeatherMapApi;
use crate::routes::forecast::forecast;
use crate::routes::home::home;
use crate::routes::locations::locations;
use crate::routes::weather::weather;

mod app_state;
mod open_weather_map;
mod routes;

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let open_weather_map_api_key = secret_store.get("OPEN_WEATHER_MAP_API_KEY").unwrap();

    let router = Router::new()
        .route("/", get(home))
        .route("/locations", post(locations))
        .route("/weather", get(weather))
        .route("/forecast", post(forecast))
        .with_state(AppState {
            open_weather_map_api: OpenWeatherMapApi::new(open_weather_map_api_key),
        });

    Ok(router.into())
}
