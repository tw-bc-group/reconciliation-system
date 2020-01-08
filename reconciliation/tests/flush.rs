use std::path::Path;

use reconciliation::plugin::prelude::*;

#[test]
fn test_flush_plugin() {
    let plugins = load_plugins(Path::new("tests").join("plugin")).unwrap();
    assert_eq!(plugins.len(), 1);
    let new_json = plugins[0]
        .flush(json!({"first_name": "ming", "last_name": "xiao"}))
        .unwrap();
    assert_eq!(new_json, json!({"full_name": "xiao ming"}));
}
