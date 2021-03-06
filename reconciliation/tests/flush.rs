use std::{fs::File, io::BufReader, path::Path};

use reconciliation::prelude::*;
use serde_json::Value;

#[test]
fn test_flush() {
    let plugin_path = Path::new("tests").join("plugin");
    let mock_data_path = Path::new("tests").join("mock_data");

    let plugins = load_plugins(plugin_path).unwrap();
    assert_eq!(plugins.len(), 2);

    for plugin in plugins {
        let file = File::open(mock_data_path.join(plugin.name()).with_extension("json")).unwrap();
        let buf_reader = BufReader::new(file);
        let stream = serde_json::Deserializer::from_reader(buf_reader).into_iter::<Value>();
        let raw_data = stream
            .flat_map(|res| -> Result<_, serde_json::Error> { Ok(res?) })
            .collect::<Vec<Value>>();

        let flush_data = raw_data
            .clone()
            .into_iter()
            .flat_map(|v| plugin.flush(v).unwrap())
            .collect::<Vec<FlushData>>();

        let file = File::open(
            mock_data_path
                .join(format!("{}_flush", plugin.name()))
                .with_extension("json"),
        )
        .unwrap();
        let buf_reader = BufReader::new(file);
        let expect_data: Vec<FlushData> = serde_json::from_reader(buf_reader).unwrap();
        assert_eq!(flush_data, expect_data);
    }
}
