use tarantool::lua_state;

use crate::errors::MeteoError;

pub fn make_id(sharding_key: &str) -> Result<i64, MeteoError> {
    let lua = lua_state();
    let bucket_id: i64 = lua
        .eval(&format!(
            "return require('vshard').router.bucket_id_strcrc32('{}')",
            sharding_key
        ))
        .map_err(|err| MeteoError::LuaError(format!("bucket_id computation failed: {err:?}")))?;

    Ok(bucket_id)
}
