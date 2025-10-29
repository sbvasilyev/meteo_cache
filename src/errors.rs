use serde::{Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum MeteoError {
    #[error("[weather_service] error: `{0}`")]
    WeatherServiceError(String),

    #[error("[bad_request] error: `{0}`")]
    RequestError(String),

    #[error("[tarantool] error: ")]
    TarantoolError(
        #[from]
        #[serde(serialize_with = "as_display")]
        tarantool::error::Error,
    ),

    #[error("[serde_json] error: ")]
    SerdeError(
        #[from]
        #[serde(serialize_with = "as_display")]
        serde_json::Error,
    ),
}

fn as_display<T, S>(value: &T, ser: S) -> Result<S::Ok, S::Error>
where
    T: std::fmt::Display,
    S: Serializer,
{
    ser.serialize_str(&value.to_string())
}
