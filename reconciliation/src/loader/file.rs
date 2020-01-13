use std::{fs::File, path::Path};

use super::*;
use anyhow::Result;

pub struct FileLoader<P: AsRef<Path>> {
    path: P,
}

impl<P> FileLoader<P>
where
    P: AsRef<Path>,
{
    pub fn new(path: P) -> FileLoader<P> {
        FileLoader { path }
    }
}

impl<P> Loader<File> for FileLoader<P>
where
    P: AsRef<Path>,
{
    fn get(&self, name: &str, _start: i64, _end: i64) -> Result<File> {
        File::open(&self.path).map_err(Into::into)
    }
}
