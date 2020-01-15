use std::{io::Read, path::Path};

use rayon::{ThreadPool, ThreadPoolBuilder};
use reconciliation::prelude::*;

pub(crate) struct JobManager<R: Read, L: Loader<R> + Sync> {
    system: System<R, L>,
    thread_pool: ThreadPool,
}

impl<R, L> JobManager<R, L>
where
    R: Read,
    L: Loader<R> + Sync,
{
    pub fn new<P: AsRef<Path>>(l: L, p: P) -> JobManager<R, L> {
        JobManager {
            system: System::init(l, p).expect("failed to initialize reconciliation system"),
            thread_pool: ThreadPoolBuilder::default()
                .build()
                .expect("failed to initialize thread pool"),
        }
    }

    pub fn run(&self) {
        let res = self.system.process(0, 0);
        println!("res: {:?}", res);
    }
}
