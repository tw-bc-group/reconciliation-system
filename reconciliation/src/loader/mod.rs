mod file;

pub mod prelude {
    pub use super::file::*;
}

use std::io::Read;

use anyhow::Result;

pub trait Loader<R: Read> {
    fn get(&self, name: &str, start: i64, end: i64) -> Result<R>;
}
