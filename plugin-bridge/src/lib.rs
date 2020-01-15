#[macro_use]
extern crate serde;

mod bridge;

use std::convert::TryInto;

use crate::bridge::*;
use reconciliation::prelude::*;

#[derive(Default)]
struct BridgePlugin;

impl Flush for BridgePlugin {
    fn name(&self) -> &'static str {
        "bridge"
    }

    fn groups(&self) -> Vec<&'static str> {
        vec!["bridge_and_payment"]
    }

    fn flush(&self, json: Value) -> Result<Vec<FlushData>> {
        serde_json::from_value::<Bridge>(json)
            .map_err(Into::into)
            .and_then(|bridge| bridge.try_into())
    }
}

declare_flush_plugin!(BridgePlugin::default);
