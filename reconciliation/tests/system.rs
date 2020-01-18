use std::{ops::Range, path::Path};

use chrono::{Duration, TimeZone, Utc};
use reconciliation::prelude::*;

#[test]
pub fn test_system() {
    let mock_data_path = Path::new("tests").join("mock_data");
    let plugin_path = Path::new("tests").join("plugin");
    let system = System::init(FileLoader::new(mock_data_path), plugin_path).unwrap();
    let start = Utc.ymd(2019, 12, 15).and_hms(16, 0, 0);
    let end = Utc.ymd(2019, 12, 16).and_hms(16, 0, 0);
    let buffer = Duration::hours(1);
    let res = system
        .process(
            Range {
                start: start - buffer,
                end: end + buffer,
            },
            Range { start, end },
        )
        .unwrap();
    assert_eq!(res.get("bridge_and_payment").unwrap().len(), 5);
}
