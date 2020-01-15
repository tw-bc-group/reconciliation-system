mod file;
mod http;

pub mod prelude {
    pub use super::{file::*, http::*};
}

use std::io::Read;

use anyhow::Result;

pub trait Loader<R: Read> {
    fn get(&self, name: &str, start: i64, end: i64) -> Result<R>;
}
