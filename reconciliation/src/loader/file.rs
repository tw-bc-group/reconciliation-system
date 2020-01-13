use std::{fmt::Debug, fs::File, path::Path};

use super::*;
use anyhow::Result;

pub struct FileLoader<P: AsRef<Path> + Debug> {
    path: P,
}

impl<P> FileLoader<P>
where
    P: AsRef<Path> + Debug,
{
    pub fn new(path: P) -> FileLoader<P> {
        FileLoader { path }
    }
}

impl<P> Loader<File> for FileLoader<P>
where
    P: AsRef<Path> + Debug,
{
    fn get(&self, name: &str, _start: i64, _end: i64) -> Result<File> {
        File::open(self.path.as_ref().join(name).with_extension("json")).map_err(Into::into)
    }
}
