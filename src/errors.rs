use thiserror::Error;

#[derive(Debug, Error)]
pub enum MeteoError {
    #[error("[weather_service] error: `{0}`")]
    WeatherServiceError(String),

    #[error("[bad_request] error: `{0}`")]
    RequestError(String),

    #[error("[tarantool] error: ")]
    TarantoolError(#[from] tarantool::error::Error),

    #[error("[serde_json] error: ")]
    SerdeError(#[from] serde_json::Error),
}
