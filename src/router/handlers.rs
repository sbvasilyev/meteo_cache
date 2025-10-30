use std::collections::HashMap;

use shors::transport::{Context, http::Request, rpc::client::Builder};

use crate::{errors::MeteoError, lua_helpers, om_service::OpenMeteoService};

pub fn ping_handler(_ctx: &mut Context, _req: Request) -> Result<String, MeteoError> {
    Ok("pong".into())
}

pub fn forecast_handler(
    open_meteo: OpenMeteoService,
    ctx: &mut Context,
    req: Request,
) -> Result<String, MeteoError> {
    let params = parse_query(&req.query);

    let city = params
        .get("city")
        .ok_or(MeteoError::RequestError("'city' is not in query".into()))?
        .to_owned();
    let query = make_consistent_query(params);

    let key = format!("{city}:::{query}");
    let bucket_id = lua_helpers::make_id(&key)?;
    let lua = tarantool::lua_state();

    let get_result: Option<String> = Builder::new(&lua)
        .shard_endpoint("get_forecast")
        .call(ctx, bucket_id, key.clone())
        .map_err(|err| MeteoError::StorageError(format!("rpc call failed: {err:?}")))?
        .get(0)
        .ok_or(MeteoError::StorageError("forecast decode fail".into()))?;
    if let Some(forecast) = get_result {
        return Ok(forecast);
    }

    let weather = open_meteo.get_weather(&city, &query)?;

    let _put_result = Builder::new(&lua)
        .shard_endpoint("put_forecast")
        .call(ctx, bucket_id, (bucket_id, key, weather.clone()))
        .map_err(|err| MeteoError::StorageError(format!("rpc call failed: {err:?}")))?;

    Ok(weather)
}

fn parse_query(query: &str) -> HashMap<String, String> {
    url::form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .collect()
}

fn make_consistent_query(mut params: HashMap<String, String>) -> String {
    params.remove("city");

    let mut pairs: Vec<(&str, &str)> = params
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    pairs.sort_unstable_by(|(ka, _), (kb, _)| ka.cmp(kb));

    url::form_urlencoded::Serializer::new(String::new())
        .extend_pairs(pairs)
        .finish()
}
