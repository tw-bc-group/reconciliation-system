use std::{convert::TryFrom, str::FromStr};

use super::*;

use anyhow::{Error, Result};
use reconciliation::plugin::prelude::*;

fn bridge_in_tx_id(tx_id: &str, position: i32) -> String {
    format!(
        "{:x}",
        md5::compute(format!("{}-{}", tx_id, position).as_bytes())
    )
}

impl TryFrom<Bridge> for Vec<FlushData> {
    type Error = Error;

    fn try_from(bridge: Bridge) -> Result<Vec<FlushData>> {
        let mut res = Vec::new();
        match bridge {
            Bridge::In(bridge_in) => {
                let tx_id = bridge_in.tx_id;
                for v in bridge_in.vout {
                    res.push(FlushData {
                        tx_id: bridge_in_tx_id(&tx_id, v.position),
                        amount: Amount::from_str(&v.amount)?,
                        address: v.address,
                        currency: bridge_in.coin_type.clone(),
                        direction: Direction::In,
                        ..Default::default()
                    })
                }
            }
            Bridge::Out(bridge_out) => {
                let tx_id = bridge_out.tx_id;
                for mut transaction in bridge_out.transactions {
                    assert_eq!(transaction.vout.len(), 1);
                    let vout = transaction.vout.pop().unwrap();
                    res.push(FlushData {
                        tx_id: tx_id.clone(),
                        amount: Amount::from_str(&vout.amount)?,
                        address: vout.address,
                        currency: transaction.coin_type,
                        direction: Direction::Out,
                        ..Default::default()
                    })
                }
            }
        }
        Ok(res)
    }
}
