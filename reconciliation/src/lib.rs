#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
pub mod macros;
pub mod models;
pub mod plugin;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
