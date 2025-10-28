use serde::Serialize;
use shors::transport::{
    Context,
    http::{Request, Response, route::Builder},
};
use std::error::Error;
use std::os::raw::c_int;
use tarantool::ffi::lua as ffi_lua;
use tarantool::tlua;

#[derive(Serialize)]
struct GetPingResponse {
    result: String,
}

#[tarantool::proc]
fn http_init() -> tarantool::Result<()> {
    let server = shors::transport::http::server::Server::new();
    let route_group = Builder::new().with_path("/rs").group();

    let get_ping = route_group
        .builder()
        .with_method("GET")
        .with_path("/ping")
        .build(
            |_ctx: &mut Context, _req: Request| -> Result<_, Box<dyn Error>> {
                Ok(Response::from(GetPingResponse {
                    result: "pong".into(),
                }))
            },
        );

    server.register(Box::new(get_ping));

    Ok(())
}

#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_libmeteo(l: *mut ffi_lua::lua_State) -> c_int {
    unsafe {
        let lua = tlua::StaticLua::from_static(l);
        shors::init_lua_functions(&lua).unwrap();
    }
    1
}
