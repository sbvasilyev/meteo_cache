use std::os::raw::c_int;

use shors::transport::rpc::{route::Builder, server::Server};
use tarantool::tuple::{FunctionArgs, FunctionCtx};

mod handlers;
mod init;

pub use init::create_meteo_space;

thread_local! {
    pub static RPC_SERVER: once_cell::unsync::Lazy<Server> = once_cell::unsync::Lazy::new(Server::new);
}

#[tarantool::proc]
pub fn rpc_init() {
    let route_group = Builder::new().group();

    let put_forecast = route_group
        .builder()
        .with_path("put_forecast")
        .build(handlers::put_forecast);

    let get_forecast = route_group
        .builder()
        .with_path("get_forecast")
        .build(handlers::get_forecast);

    RPC_SERVER.with(|server| {
        server.register(Box::new(put_forecast));
        server.register(Box::new(get_forecast));
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn storage_rpc_handler(ctx: FunctionCtx, args: FunctionArgs) -> c_int {
    RPC_SERVER.with(|srv| srv.handle(ctx, args))
}
