use std::{io::Read, path::Path};

use reconciliation::prelude::*;

pub(crate) struct JobManager<R: Read, L: Loader<R> + Sync> {
    system: System<R, L>,
}

impl<R, L> JobManager<R, L>
where
    R: Read,
    L: Loader<R> + Sync,
{
    pub fn new<P: AsRef<Path>>(l: L, p: P) -> JobManager<R, L> {
        JobManager {
            system: System::init(l, p).expect("failed to initialize reconciliation system"),
        }
    }

    pub fn run(&self) {
        let res = self.system.process(0, 0);
        println!("res: {:?}", res);
    }
}
