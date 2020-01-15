use std::{convert::TryFrom, str::FromStr};

use super::*;

use anyhow::Error;
use reconciliation::plugin::prelude::*;

// fn payment_in_tx_id(tx_id: &str, position: i32) -> String {
//     format!(
//         "{:x}",
//         md5::compute(format!("{}-{}", tx_id, position).as_bytes())
//     )
// }

impl TryFrom<Payment> for Vec<FlushData> {
    type Error = Error;

    fn try_from(payment: Payment) -> Result<Vec<FlushData>> {
        let mut res = Vec::new();
        match payment {
            Payment::In(payment_in) => {
                let tx_id = payment_in.serial_number;
                res.push(FlushData {
                    tx_id,
                    amount: Amount::from_str(&payment_in.amount)?,
                    address: payment_in.address,
                    currency: payment_in.currency.clone(),
                    direction: Direction::In,
                    ..Default::default()
                })
            }
            Payment::Out(payment_out) => {
                let tx_id = payment_out.serial_number;
                res.push(FlushData {
                    tx_id,
                    amount: Amount::from_str(&payment_out.amount)?,
                    address: payment_out.address,
                    currency: payment_out.currency.clone(),
                    direction: Direction::In,
                    ..Default::default()
                })
            }
        }
        Ok(res)
    }
}
