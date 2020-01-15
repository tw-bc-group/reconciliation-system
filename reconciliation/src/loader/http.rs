use std::borrow::Cow;

use super::*;
use anyhow::Result;
use reqwest::blocking::{Client, Response};
use url::Url;

pub struct HttpLoader {
    url: Url,
}

impl HttpLoader {
    pub fn new<U: Into<Url>>(u: U) -> HttpLoader {
        HttpLoader { url: u.into() }
    }
}

impl Loader<Response> for HttpLoader {
    fn get(&self, name: &str, start: i64, end: i64) -> Result<Response> {
        serde_urlencoded::to_string(&[
            ("name", Cow::from(name)),
            ("start", Cow::from(start.to_string())),
            ("end", Cow::from(end.to_string())),
        ])
        .map_err(Into::into)
        .and_then(|query| {
            let mut url = self.url.clone();
            url.set_query(Some(&query));
            Client::new()
                .get(self.url.as_ref())
                .send()
                .map_err(Into::into)
        })
    }
}
