use std::path::Path;
use reconciliation_system::plugin::prelude::*;

//may be cron job or http gateway
fn main() {
    load_plugins(Path::new("plugins").join("flush")).unwrap();
    println!("demo finish");
}