use std::{
    cmp::{Eq, PartialEq},
    hash::{Hash, Hasher},
};

use super::amount::*;
use serde_json::Value;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Mismatch {
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
    pub tx_id: String,
    pub amount: Amount,
    pub address: String,
    pub currency: String,
    pub direction: Direction,
    pub raw_data: Option<Value>,
}

impl FlushData {
    pub fn diff(&self, other: &FlushData) -> Vec<Mismatch> {
        let mut mismatch = Vec::new();

        if self.amount != other.amount {
            mismatch.push(Mismatch::Amount);
        }

        if self.address != other.address {
            mismatch.push(Mismatch::Address);
        }

        if self.currency != other.currency {
            mismatch.push(Mismatch::Currency);
        }

        if self.direction != other.direction {
            mismatch.push(Mismatch::Direction);
        }

        mismatch
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
