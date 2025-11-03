use tarantool::{
    lua_state,
    tlua::{LuaFunction, StaticLua},
};

use crate::errors::MeteoError;

pub fn init_lua_fn(lua: &StaticLua) {
    if let Err(err) = lua.exec(
        "bucket_id_fn = function(...) return require('vshard').router.bucket_id_strcrc32(...) end",
    ) {
        eprintln!("init bucket_id_fn failed: {err:?}");
    }
}

pub fn make_id(sharding_key: &str) -> Result<i64, MeteoError> {
    let lua = lua_state();
    let func = lua
        .get::<LuaFunction<_>, _>("bucket_id_fn")
        .ok_or(MeteoError::LuaError("bucket_id_fn failed".into()))?;

    let bucket_id = func
        .call_with_args(sharding_key)
        .map_err(|err| MeteoError::LuaError(format!("bucket_id_fn error: {err:?}")))?;
    Ok(bucket_id)
}
