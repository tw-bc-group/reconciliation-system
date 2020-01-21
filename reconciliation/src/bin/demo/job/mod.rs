mod queue;

pub(crate) mod prelude {
    pub(crate) use super::queue::*;
}

use std::{io::Read, ops::Range, path::Path};

use crate::error::*;
use chrono::{DateTime, Duration, TimeZone, Utc};

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

    pub(crate) fn id(&self) -> String {
        let format = "%Y%m%d";
        let duration = Duration::hours(8);
        format!(
            "{}-{}-{}",
            (self.start + duration).format(format),
            (self.end + duration).format(format),
            Utc::now().timestamp_millis(),
        )
    }

    pub(crate) fn with_buffer(&self) -> Range<DateTime<Utc>> {
        Range {
            start: self.start - self.buffer,
            end: self.end + self.buffer,
        }
    }

    pub(crate) fn without_buffer(&self) -> Range<DateTime<Utc>> {
        Range {
            start: self.start,
            end: self.end,
        }
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
        let id = time.id();
        Ok(Job { id, time })
    }
}
