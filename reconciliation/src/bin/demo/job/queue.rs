use std::{collections::HashMap, thread};

use super::*;
use crate::error::*;
use crossbeam_channel::Sender;
use reconciliation::prelude::*;

pub(crate) enum JobQueueMessage {
    Job(Job),
    Stop,
}

pub(crate) struct JobQueue {
    tx: Sender<JobQueueMessage>,
    handle: Option<thread::JoinHandle<()>>,
}

fn write_res_to_excel(job_id: &str, res: HashMap<&str, Vec<StatementResult>>) -> Result<()> {
    let mut excel = Excel::new(format!("excel/{}.xlxs", job_id));
    for (name, data) in res {
        excel.write_sheet(name, &data)?;
    }
    excel.save().map_err(Into::into)
}

impl JobQueue {
    pub fn new<R, L, P>(l: L, p: P) -> JobQueue
    where
        R: Read + Sync + Send + 'static,
        L: Loader<R> + Sync + Send + 'static,
        P: AsRef<Path>,
    {
        let system = System::init(l, p).expect("failed to initialize reconciliation system");

        let (tx, rx) = crossbeam_channel::bounded::<JobQueueMessage>(16);

        let handle = thread::spawn(move || loop {
            match rx.recv() {
                Ok(msg) => match msg {
                    JobQueueMessage::Job(job) => {
                        debug!("A new job coming: {:?}", job);
                        let (start, end) = job.time.buffer_time();
                        match system.process(start, end) {
                            Ok(res) => {
                                if let Err(err) = write_res_to_excel(&job.id, res) {
                                    warn!("failed to write or save excel, {:?}", err);
                                }
                            }
                            Err(err) => {
                                warn!("system process error, {:?}", err);
                            }
                        }
                    }
                    JobQueueMessage::Stop => break,
                },
                Err(err) => {
                    panic!("job queue failed to receive a new job, {:?}", err);
                }
            }
        });

        JobQueue {
            tx,
            handle: Some(handle),
        }
    }

    pub fn new_job(&self, start: i64, end: i64) -> Result<String> {
        Job::new(start, end).and_then(|job| {
            let id = job.id.clone();
            self.tx
                .try_send(JobQueueMessage::Job(job))
                .map_err(Into::into)
                .map(|_| id)
        })
    }
}

impl Drop for JobQueue {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            if self.tx.send(JobQueueMessage::Stop).is_ok() {
                handle.join().unwrap();
            }
        }
    }
}
