use std::{path::Path, convert::AsRef};

use anyhow::Result;
use serde_json::Value;

pub trait Flush {
    fn name(&self) -> &'static str;
    fn group(&self) -> &'static str;
    fn rebuild_json(&self, json: Value) -> Result<Value>;
}

pub fn load_plugins<P: AsRef<Path>>(dir: P) -> Result<()> {
    super::plugin_list(dir).and_then(|_plugins| Ok(()))
}
