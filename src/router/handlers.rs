use std::collections::HashMap;

use shors::transport::{Context, http::Request};

use crate::{errors::MeteoError, om_service::OpenMeteoService};

pub fn ping_handler(_ctx: &mut Context, _req: Request) -> Result<String, MeteoError> {
    Ok("pong".into())
}

pub fn forecast_handler(
    open_meteo: OpenMeteoService,
    _ctx: &mut Context,
    req: Request,
) -> Result<String, MeteoError> {
    let params = parse_query(&req.query);

    let city = params
        .get("city")
        .ok_or(MeteoError::RequestError("'city' is not in query".into()))?
        .to_owned();
    let query = make_consistent_query(params);

    open_meteo.get_weather(&city, &query)
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
