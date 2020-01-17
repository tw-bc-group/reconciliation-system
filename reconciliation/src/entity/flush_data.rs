use std::{
    cmp::{Eq, PartialEq},
    hash::{Hash, Hasher},
};

use super::{amount::*, direction::*, time::*};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FlushDataMismatch {
    Amount,
    CrossDate,
    Currency,
    Direction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FlushData {
    pub tx_id: String,
    pub amount: Amount,
    pub address: String,
    pub currency: String,
    pub direction: Direction,
    pub datetime: TransactionTime,
    pub raw_data: Option<Value>,
    #[serde(default)]
    #[serde(skip_serializing)]
    pub belongs: String,
}

impl Default for FlushData {
    fn default() -> Self {
        FlushData {
            tx_id: String::default(),
            amount: Amount::default(),
            address: String::default(),
            currency: String::default(),
            direction: Direction::default(),
            datetime: TransactionTime::default(),
            raw_data: None,
            belongs: String::default(),
        }
    }
}

impl FlushData {
    pub fn fields() -> &'static [&'static str] {
        &["流水号", "地址", "金额", "币种", "方向", "交易时间"]
    }
    pub fn id(&self) -> String {
        format!("{}|{}", self.tx_id, self.address)
    }
    pub fn compare(&self, other: &FlushData) -> Vec<FlushDataMismatch> {
        let mut mismatches = Vec::new();

        if self.amount != other.amount {
            mismatches.push(FlushDataMismatch::Amount);
        }

        if self.currency != other.currency {
            mismatches.push(FlushDataMismatch::Currency);
        }

        if self.direction != other.direction {
            mismatches.push(FlushDataMismatch::Direction);
        }

        mismatches
    }
}

impl Eq for FlushData {}

impl PartialEq for FlushData {
    fn eq(&self, other: &FlushData) -> bool {
        self.tx_id == other.tx_id
            && self.amount == other.amount
            && self.address == other.address
            && self.currency == other.currency
            && self.direction == other.direction
    }
}

impl Hash for FlushData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tx_id.hash(state);
        self.amount.hash(state);
        self.address.hash(state);
        self.currency.hash(state);
        self.direction.hash(state);
    }
}
