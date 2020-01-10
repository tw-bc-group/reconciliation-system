#[macro_use]
extern crate serde;

mod brige;

use std::convert::TryInto;

use crate::brige::*;
use reconciliation::{declare_flush_plugin, plugin::prelude::*};

#[derive(Default)]
struct BrigePlugin;

impl Flush for BrigePlugin {
    fn name(&self) -> &'static str {
        "brige"
    }

    fn group(&self) -> &'static str {
        "brige_and_revenue"
    }

    fn flush(&self, json: Value) -> Result<Vec<FlushData>> {
        serde_json::from_value::<Brige>(json)
            .map_err(Into::into)
            .and_then(|brige| brige.try_into())
    }
}

declare_flush_plugin!(BrigePlugin::default);
