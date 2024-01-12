use crate::open_weather_map::open_weather_map_api::OpenWeatherMapApi;

#[derive(Clone)]
pub struct AppState {
    pub open_weather_map_api: OpenWeatherMapApi,
}
