use std::convert::TryFrom;

use super::*;

use anyhow::Error;
use reconciliation::plugin::prelude::*;

fn brige_in_tx_id(tx_id: &str, position: i32) -> String {
    format!(
        "{:x}",
        md5::compute(format!("{}-{}", tx_id, position).as_bytes())
    )
}

impl TryFrom<Brige> for Vec<FlushData> {
    type Error = Error;

    fn try_from(brige: Brige) -> Result<Vec<FlushData>> {
        let mut res = Vec::new();
        match brige {
            Brige::In(brige_in) => {
                let tx_id = brige_in.tx_id;
                for v in brige_in.vout {
                    res.push(FlushData {
                        tx_id: brige_in_tx_id(&tx_id, v.position),
                        amount: Amount::from_str(&v.amount)?,
                        address: v.address,
                        currency: brige_in.coin_type.clone(),
                        direction: Direction::In,
                        ..Default::default()
                    })
                }
            }
            Brige::Out(brige_out) => {
                let tx_id = brige_out.tx_id;
                for mut transaction in brige_out.transactions {
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
