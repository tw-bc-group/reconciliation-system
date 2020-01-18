use chrono::{Datelike, NaiveDateTime, Timelike, Utc};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransactionTime {
    inner: NaiveDateTime,
}

impl Default for TransactionTime {
    fn default() -> Self {
        TransactionTime {
            inner: Utc::now().naive_local(),
        }
    }
}

impl From<NaiveDateTime> for TransactionTime {
    fn from(inner: NaiveDateTime) -> Self {
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
