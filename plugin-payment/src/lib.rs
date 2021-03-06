#[macro_use]
extern crate serde;

mod payment;

use std::convert::TryInto;

use crate::payment::*;
use anyhow::Result;
use reconciliation::prelude::*;

#[derive(Default)]
struct PaymentPlugin;

impl Flush for PaymentPlugin {
    fn name(&self) -> &'static str {
        "payment"
    }

    fn groups(&self) -> Vec<&'static str> {
        vec!["bridge_and_payment"]
    }

    fn flush(&self, json: Value) -> Result<Vec<FlushData>> {
        serde_json::from_value::<Payment>(json)
            .map_err(Into::into)
            .and_then(|payment| payment.try_into())
    }
}

declare_flush_plugin!(PaymentPlugin::default);
