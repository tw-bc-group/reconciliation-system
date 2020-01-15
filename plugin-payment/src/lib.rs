#[macro_use]
extern crate serde;

mod payment;

use reconciliation::prelude::*;
use serde_json::Value;

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
        serde_json::from_value::<FlushData>(json)
            .map_err(Into::into)
            .map(|v| vec![v])
    }
}

declare_flush_plugin!(PaymentPlugin::default);
