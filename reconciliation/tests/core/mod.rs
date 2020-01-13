use std::path::Path;

use chrono::Utc;
use reconciliation::prelude::*;

#[test]
pub fn test_core() {
    let mock_data_path = Path::new("tests").join("mock_data");
    let plugin_path = Path::new("tests").join("flush").join("plugin");

    let start = Utc::now().timestamp_millis();
    let mut core = Core::new(FileLoader::new(mock_data_path), plugin_path);
    let res = core.run(start, start).unwrap();
    //assert_eq!(res.get("bridge_and_account").unwrap().len(), 3);
    assert_eq!(res.get("bridge_and_account").unwrap().len(), 7);
}
