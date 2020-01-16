mod queue;

pub(crate) mod prelude {
    pub(crate) use super::queue::*;
}

use std::{io::Read, path::Path};

use crate::error::*;
use chrono::{DateTime, Duration, TimeZone, Utc};
use textnonce::TextNonce;

#[derive(Debug)]
pub(crate) struct JobTime {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    buffer: Duration,
}

impl JobTime {
    pub(crate) fn new(start: i64, end: i64, buffer: Duration) -> JobTime {
        JobTime {
            start: Utc.timestamp_millis(start),
            end: Utc.timestamp_millis(end),
            buffer,
        }
    }

    pub(crate) fn id(&self) -> Result<String> {
        TextNonce::sized(16).map_err(Error::TextNonce).map(|nonce| {
            let format = "%Y%m%d";
            let duration = Duration::hours(8);

            format!(
                "{}-{}-{}",
                (self.start + duration).format(format),
                (self.end + duration).format(format),
                nonce,
            )
        })
    }

    pub(crate) fn buffer_time(&self) -> (i64, i64) {
        let start = self.start - self.buffer;
        let end = self.end - self.buffer;
        (start.timestamp_millis(), end.timestamp_millis())
    }
}

#[derive(Debug)]
pub(crate) struct Job {
    pub id: String,
    pub time: JobTime,
}

impl Job {
    pub fn new(start: i64, end: i64) -> Result<Job> {
        let time = JobTime::new(start, end, Duration::hours(1));
        let id = time.id()?;
        Ok(Job { id, time })
    }
}
