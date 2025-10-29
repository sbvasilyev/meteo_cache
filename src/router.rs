use shors::transport::{
    Context,
    http::{Request, route::Builder, server::Server},
};

use crate::om_service::OpenMeteoService;

mod handlers;

#[tarantool::proc]
pub fn http_init() -> tarantool::Result<()> {
    let server = Server::new();
    let route_group = Builder::new().with_path("/rs").group();

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
