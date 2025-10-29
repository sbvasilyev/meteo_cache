use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForecastEntry {
    pub key: String,
    pub bucket_id: i64,
    pub value: String,
    pub expires_at: u64,
}
impl tarantool::tuple::Encode for ForecastEntry {}
