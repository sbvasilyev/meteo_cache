use fibreq::Client;
use serde_json::Value;

use crate::errors::MeteoError;

struct Coords {
    lat: f64,
    lon: f64,
}

pub struct OpenMeteoService {
    http_client: Client,
    geocode_url: String,
    forecast_url: String,
}

impl Default for OpenMeteoService {
    fn default() -> Self {
        Self {
            http_client: fibreq::ClientBuilder::new().build(),
            geocode_url: "https://geocoding-api.open-meteo.com/v1/search?count=1&language=en&name="
                .into(),
            forecast_url: "https://api.open-meteo.com/v1/forecast?".into(),
        }
    }
}

impl OpenMeteoService {
    pub fn get_weather(&self, city: &str, query: &str) -> Result<String, MeteoError> {
        let coords = self.geocode_city(city)?;
        self.forecast(coords, query)
    }

    fn geocode_city(&self, city: &str) -> Result<Coords, MeteoError> {
        let req_ulr = format!("{}{}", self.geocode_url, city);

        let resp_body = self.make_request(&req_ulr)?;
        let json_body: Value = serde_json::from_str(&resp_body).map_err(|err| {
            MeteoError::WeatherServiceError(format!("geocode body parsing error: {err:?}"))
        })?;

        let result_entry = json_body
            .get("results")
            .and_then(|results| results.as_array())
            .and_then(|results| results.first())
            .ok_or(MeteoError::WeatherServiceError("empty coordinates".into()))?;

        let lat = get_coord(result_entry, "latitude")?;
        let lon = get_coord(result_entry, "longitude")?;
        Ok(Coords { lat, lon })
    }

    fn forecast(&self, coords: Coords, query: &str) -> Result<String, MeteoError> {
        let req_ulr = format!(
            "{}latitude={}&longitude={}&{}",
            self.forecast_url, coords.lat, coords.lon, query
        );

        self.make_request(&req_ulr)
    }

    fn make_request(&self, url: &str) -> Result<String, MeteoError> {
        let mut resp = self
            .http_client
            .get(url)
            .map_err(|err| MeteoError::WeatherServiceError(format!("request failed: {err:?}")))?
            .send()
            .map_err(|err| MeteoError::WeatherServiceError(format!("request failed: {err:?}")))?;

        if resp.status() != 200 {
            return Err(MeteoError::WeatherServiceError(format!(
                "request failed: status {}",
                resp.status()
            )));
        }

        resp.text()
            .map_err(|err| MeteoError::WeatherServiceError(format!("request failed: {err:?}")))
    }
}

fn get_coord(entry: &Value, coord: &str) -> Result<f64, MeteoError> {
    entry
        .get("latitude")
        .and_then(|l| l.as_f64())
        .ok_or(MeteoError::WeatherServiceError(format!(
            "{coord} is not found in result"
        )))
}
