use std::os::raw::c_int;
use tarantool::ffi::lua as ffi_lua;
use tarantool::tlua;

mod bucket_id;
mod errors;
mod models;
mod om_service;
mod router;
mod storage;

pub use router::http_init;
pub use storage::{rpc_init, storage_rpc_handler};

#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_libmeteo(l: *mut ffi_lua::lua_State) -> c_int {
    let lua = unsafe { tlua::StaticLua::from_static(l) };

    shors::init_lua_functions(&lua).unwrap();
    bucket_id::init_lua_fn(&lua);

    return 1;
}
