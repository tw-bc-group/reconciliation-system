use std::{convert::AsRef, path::Path};

use crate::entity::prelude::*;

use anyhow::Result;
use serde_json::Value;

pub const PROBE_PLUGIN_FUNCTION: &str = "probe_plugin";

pub trait Flush {
    fn name(&self) -> &'static str;
    fn group(&self) -> &'static str;
    fn flush(&self, json: Value) -> Result<Vec<FlushData>>;
}

pub fn load_plugins<P: AsRef<Path>>(dir: P) -> Result<Vec<Box<dyn Flush>>> {
    plugin_load!(dir, PROBE_PLUGIN_FUNCTION.as_bytes(), Flush)
}
