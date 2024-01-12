use crate::open_weather_map::air_quality_response::AirQualityResponse;
use crate::open_weather_map::five_days_forecast_response::FiveDaysForecastResponse;
use crate::open_weather_map::geocoding_response::GeocodingResponse;
use crate::open_weather_map::weather_response::WeatherResponse;

#[derive(Clone)]
pub struct OpenWeatherMapApi {
    http_client: reqwest::Client,
    api_key: String,
}

impl OpenWeatherMapApi {
    pub fn new(api_key: String) -> Self {
        OpenWeatherMapApi {
            http_client: reqwest::Client::new(),
            api_key,
        }
    }

    pub async fn get_geocoding(&self, query: String) -> Result<GeocodingResponse, reqwest::Error> {
        let params = [
            ("appid", self.api_key.clone()),
            ("limit", "5".into()),
            ("q", query),
        ];

        reqwest::Client::new()
            .get("http://api.openweathermap.org/geo/1.0/direct")
            .query(&params)
            .send()
            .await?
            .json::<GeocodingResponse>()
            .await
    }

    pub async fn get_weather(
        &self,
        (lat, lon): (f64, f64),
    ) -> Result<WeatherResponse, reqwest::Error> {
        let params = [
            ("appid", self.api_key.clone()),
            ("lat", lat.to_string()),
            ("lon", lon.to_string()),
            ("units", "metric".into()),
        ];

        self.http_client
            .get("https://api.openweathermap.org/data/2.5/weather")
            .query(&params)
            .send()
            .await?
            .json::<WeatherResponse>()
            .await
    }

    pub async fn get_air_quality(
        &self,
        (lat, lon): (f64, f64),
    ) -> Result<AirQualityResponse, reqwest::Error> {
        let params = [
            ("appid", self.api_key.clone()),
            ("lat", lat.to_string()),
            ("lon", lon.to_string()),
        ];

        self.http_client
            .get(" https://api.openweathermap.org/data/2.5/air_pollution")
            .query(&params)
            .send()
            .await?
            .json::<AirQualityResponse>()
            .await
    }

    pub async fn get_five_days_forecast(
        &self,
        (lat, lon): (f64, f64),
    ) -> Result<FiveDaysForecastResponse, reqwest::Error> {
        let params = [
            ("appid", self.api_key.clone()),
            ("lat", lat.to_string()),
            ("lon", lon.to_string()),
            ("units", "metric".into()),
        ];

        self.http_client
            .get(" https://api.openweathermap.org/data/2.5/forecast")
            .query(&params)
            .send()
            .await?
            .json::<FiveDaysForecastResponse>()
            .await
    }
}
