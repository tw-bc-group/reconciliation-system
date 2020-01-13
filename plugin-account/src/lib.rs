use reconciliation::prelude::*;
use serde_json::Value;

#[derive(Default)]
struct AccountPlugin;

impl Flush for AccountPlugin {
    fn name(&self) -> &'static str {
        "account"
    }

    fn group(&self) -> &'static str {
        "bridge_and_account"
    }

    fn flush(&self, json: Value) -> Result<Vec<FlushData>> {
        serde_json::from_value::<FlushData>(json)
            .map_err(Into::into)
            .map(|v| vec![v])
    }
}

declare_flush_plugin!(AccountPlugin::default);
