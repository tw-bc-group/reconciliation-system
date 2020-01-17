use std::convert::TryFrom;

use super::*;

use anyhow::{Error, Result};
use reconciliation::prelude::*;

impl TryFrom<Payment> for Vec<FlushData> {
    type Error = Error;

    fn try_from(payment: Payment) -> Result<Vec<FlushData>> {
        Ok(vec![FlushData {
            tx_id: payment.serial_number,
            amount: Amount::from(payment.amount),
            address: payment.address,
            currency: payment.currency.clone(),
            direction: if payment.r#type == 2 {
                Direction::Out
            } else {
                Direction::In
            },
            ..Default::default()
        }])
    }
}
