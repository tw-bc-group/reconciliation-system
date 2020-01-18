use std::convert::TryFrom;

use super::*;

use anyhow::{Error, Result};
use chrono::DateTime;
use reconciliation::prelude::*;

impl TryFrom<Payment> for Vec<FlushData> {
    type Error = Error;

    fn try_from(payment: Payment) -> Result<Vec<FlushData>> {
        Ok(vec![FlushData {
            tx_id: payment.serial_number,
            amount: Amount::from(payment.amount),
            address: payment.address,
            currency: payment.currency.clone(),
            datetime: DateTime::parse_from_rfc3339(&payment.create_time)?
                .naive_local()
                .into(),
            direction: if payment.r#type == 2 {
                Direction::Out
            } else {
                Direction::In
            },
            ..Default::default()
        }])
    }
}
