use std::os::raw::c_int;
use tarantool::ffi::lua as ffi_lua;
use tarantool::tlua;

mod config;
mod errors;
mod lua_helpers;
mod models;
mod om_service;
mod router;
mod storage;

pub use config::update_config;
pub use router::http_init;
pub use storage::{create_meteo_space, rpc_init, storage_rpc_handler};

#[tarantool::proc]
pub fn init_meteo_space() {
    if let Err(err) = create_meteo_space() {
        eprintln!("failed to init meteo space: {err:?}");
    }
}

#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_libmeteo(l: *mut ffi_lua::lua_State) -> c_int {
    let lua = unsafe { tlua::StaticLua::from_static(l) };

    shors::init_lua_functions(&lua).unwrap();
    lua_helpers::init_lua_fn(&lua);

    return 1;
}
