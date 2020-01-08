use reconciliation::{declare_flush_plugin, plugin::prelude::*};

#[derive(Default, Deserialize)]
struct Demo {
    last_name: String,
    first_name: String,
}

impl Flush for Demo {
    fn name(&self) -> &'static str {
        "demo"
    }

    fn group(&self) -> &'static str {
        "group1"
    }

    fn flush(&self, json: Value) -> Result<Value> {
        serde_json::from_value::<Demo>(json)
            .map_err(Into::into)
            .map(|demo| json!({ "full_name": format!("{} {}", demo.last_name, demo.first_name) }))
    }
}

declare_flush_plugin!(Demo::default);
