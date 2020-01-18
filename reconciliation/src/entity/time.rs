use std::ops::{Add, Sub};

use chrono::{DateTime, Datelike, Duration, Timelike, Utc};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct TransactionTime {
    inner: DateTime<Utc>,
}

impl Default for TransactionTime {
    fn default() -> Self {
        TransactionTime { inner: Utc::now() }
    }
}

impl AsRef<DateTime<Utc>> for TransactionTime {
    fn as_ref(&self) -> &DateTime<Utc> {
        &self.inner
    }
}

impl Add<Duration> for TransactionTime {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        TransactionTime {
            inner: self.inner + rhs,
        }
    }
}

impl Sub<Duration> for TransactionTime {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        TransactionTime {
            inner: self.inner - rhs,
        }
    }
}

impl From<DateTime<Utc>> for TransactionTime {
    fn from(inner: DateTime<Utc>) -> TransactionTime {
        TransactionTime { inner }
    }
}

impl TransactionTime {
    pub fn year(&self) -> i32 {
        self.inner.year()
    }

    pub fn month(&self) -> u32 {
        self.inner.month()
    }

    pub fn day(&self) -> u32 {
        self.inner.day()
    }

    pub fn hour(&self) -> u32 {
        self.inner.hour()
    }

    pub fn minute(&self) -> u32 {
        self.inner.minute()
    }

    pub fn second(&self) -> u32 {
        self.inner.second()
    }
}
