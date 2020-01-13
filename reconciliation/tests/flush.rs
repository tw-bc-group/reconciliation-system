use std::{fs::File, io::BufReader, path::Path};

use reconciliation::plugin::prelude::*;
use serde_json::Value;

#[test]
fn test_flush_plugin() {
    let plugins = load_plugins(Path::new("tests").join("plugin")).unwrap();
    assert_eq!(plugins.len(), 1);

    let file = File::open(Path::new("tests").join("bridge.json")).unwrap();
    let buf_reader = BufReader::new(file);
    let brige_data: Vec<Value> = serde_json::from_reader(buf_reader).unwrap();
    assert_eq!(brige_data.len(), 11);

    let flush_data = brige_data
        .clone()
        .into_iter()
        .flat_map(|v| plugins[0].flush(v).unwrap())
        .collect::<Vec<FlushData>>();

    assert_eq!(flush_data.len(), 12);

    let file = File::open(Path::new("tests").join("flush.json")).unwrap();
    let buf_reader = BufReader::new(file);
    let expect_data: Vec<FlushData> = serde_json::from_reader(buf_reader).unwrap();
    assert_eq!(expect_data.len(), 12);
    assert_eq!(flush_data, expect_data);
}
