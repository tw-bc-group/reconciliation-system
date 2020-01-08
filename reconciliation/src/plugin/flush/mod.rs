use std::{convert::AsRef, path::Path};

use anyhow::Result;
use libloading::{Library, Symbol};
use serde_json::Value;

pub const PROBE_PLUGIN_FUNCTION: &str = "probe_plugin";

pub trait Flush {
    fn name(&self) -> &'static str;
    fn group(&self) -> &'static str;
    fn flush(&self, json: Value) -> Result<Value>;
}

pub fn load_plugins<P: AsRef<Path>>(dir: P) -> Result<Vec<Box<dyn Flush>>> {
    type ProbePlugin = unsafe extern "C" fn() -> Box<dyn Flush>;

    super::list_dylib(dir).map(|dylib_list| {
        dylib_list.into_iter().fold(Vec::new(), |mut acc, dylib| {
            let library = match Library::new(&dylib) {
                Ok(library) => library,
                Err(err) => panic!("failed to load library, {:?}", err),
            };

            unsafe {
                let probe =
                    match library.get::<Symbol<ProbePlugin>>(PROBE_PLUGIN_FUNCTION.as_bytes()) {
                        Ok(probe) => probe,
                        Err(err) => {
                            panic!("failed to probe plugin for library {:?}, {:?}", dylib, err);
                        }
                    };
                acc.push(probe());
            }
            acc
        })
    })
}
