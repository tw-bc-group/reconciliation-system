use super::amount::*;
use serde_json::Value;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    In,
    Out,
    Unknown,
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::Unknown
    }
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FlushData {
    pub tx_id: String,
    pub amount: Amount,
    pub address: String,
    pub currency: String,
    pub direction: Direction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_data: Option<Value>,
}
