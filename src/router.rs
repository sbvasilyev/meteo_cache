use shors::transport::{
    Context,
    http::{
        Request, Response,
        route::{Builder, Handler},
        server::Server,
    },
};

use crate::{errors::MeteoError, om_service::OpenMeteoService};

mod handlers;

#[tarantool::proc]
pub fn http_init() -> tarantool::Result<()> {
    let server = Server::new();
    let route_group = Builder::new()
        .with_path("/rs")
        .with_middleware(error_middleware)
        .group();

    let ping = route_group
        .builder()
        .with_method("GET")
        .with_path("/ping")
        .build(handlers::ping_handler);

    let forecast = route_group
        .builder()
        .with_method("GET")
        .with_path("/forecast")
        .build(|ctx: &mut Context, req: Request| {
            handlers::forecast_handler(OpenMeteoService::default(), ctx, req)
        });

    server.register(Box::new(ping));
    server.register(Box::new(forecast));

    Ok(())
}

fn error_middleware(handler: Handler<MeteoError>) -> Handler<MeteoError> {
    Handler(Box::new(move |ctx, req| match handler(ctx, req) {
        Ok(resp) => Ok(resp),
        Err(err) => Ok(Response {
            status: 400,
            body: serde_json::to_vec(&err)?,
            headers: std::collections::HashMap::from([(
                "content-type".to_string(),
                "application/json; charset=utf8".to_string(),
            )]),
        }),
    }))
}
