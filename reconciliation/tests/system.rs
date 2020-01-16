use std::path::Path;

use chrono::Utc;
use reconciliation::prelude::*;

#[test]
pub fn test_system() {
    let mock_data_path = Path::new("tests").join("mock_data");
    let plugin_path = Path::new("tests").join("plugin");

    let start = Utc::now().timestamp_millis();
    let system = System::init(FileLoader::new(mock_data_path), plugin_path).unwrap();
    let res = system.process(start, start).unwrap();
    assert_eq!(res.get("bridge_and_payment").unwrap().len(), 6);
}
