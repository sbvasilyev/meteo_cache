use std::time::{SystemTime, UNIX_EPOCH};

use shors::transport::{Context, rpc::Request};
use tarantool::space::Space;

use crate::{errors::MeteoError, models::ForecastEntry};

pub fn get_forecast(_ctx: &mut Context, req: Request) -> Result<Option<String>, MeteoError> {
    let key = req
        .parse::<String>()
        .map_err(|err| MeteoError::StorageError(format!("parsing rpc request failed: {err:?}")))?;
    let space = Space::find("meteo").ok_or(MeteoError::StorageError(
        "space 'meteo' does not exist".into(),
    ))?;

    match space.get(&(key,)).map_err(|err| {
        MeteoError::StorageError(format!("getting forecast for the key failed: {err:?}"))
    })? {
        Some(forecast) => {
            let value: String = forecast.get(2).ok_or(MeteoError::StorageError(format!(
                "unable to decode forecast value"
            )))?;

            Ok(Some(value))
        }
        None => Ok(None),
    }
}

pub fn put_forecast(_ctx: &mut Context, req: Request) -> Result<(), MeteoError> {
    let (bucket_id, key, value) = req
        .parse::<(i64, String, String)>()
        .map_err(|err| MeteoError::StorageError(format!("parsing rpc request failed: {err:?}")))?;
    let space = Space::find("meteo").ok_or(MeteoError::StorageError(
        "space 'meteo' does not exist".into(),
    ))?;

    let expires_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| MeteoError::StorageError(format!("failed to get current time: {err:?}")))?
        .as_secs()
        + 60;

    space
        .insert(&ForecastEntry {
            key,
            value,
            bucket_id,
            expires_at,
        })
        .map_err(|err| {
            MeteoError::StorageError(format!("failed to insert forecast entry: {err:?}"))
        })?;

    Ok(())
}
