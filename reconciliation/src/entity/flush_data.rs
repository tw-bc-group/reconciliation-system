use std::{
    cmp::{Eq, PartialEq},
    hash::{Hash, Hasher},
};

use super::amount::*;
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FlushDataMismatch {
    Amount,
    Address,
    Currency,
    Direction,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FlushData {
    #[serde(default)]
    #[serde(skip_serializing)]
    pub name: String,
    pub tx_id: String,
    pub amount: Amount,
    pub address: String,
    pub currency: String,
    pub direction: Direction,
    pub raw_data: Option<Value>,
}

impl FlushData {
    pub fn id(&self) -> String {
        format!("{}|{}", self.tx_id, self.address)
    }
    pub fn compare(&self, other: &FlushData) -> Vec<FlushDataMismatch> {
        let mut mismatches = Vec::new();

        if self.amount != other.amount {
            mismatches.push(FlushDataMismatch::Amount);
        }

        if self.address != other.address {
            mismatches.push(FlushDataMismatch::Address);
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
