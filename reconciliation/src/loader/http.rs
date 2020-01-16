use std::borrow::Cow;

use super::*;
use anyhow::Result;
use reqwest::{
    blocking::{Client, Response},
    StatusCode,
};
use url::Url;

pub struct HttpLoader {
    url: Url,
}

impl HttpLoader {
    pub fn new(url: &str) -> Result<HttpLoader> {
        Url::parse(url)
            .map_err(Into::into)
            .map(|url| HttpLoader { url })
    }
}

impl Loader<Response> for HttpLoader {
    fn get(&self, name: &str, start: i64, end: i64) -> Result<Response> {
        serde_urlencoded::to_string(&[
            ("startTime", Cow::from(start.to_string())),
            ("endTime", Cow::from(end.to_string())),
        ])
        .map_err(Into::into)
        .and_then(|query| {
            let mut url = self.url.clone();
            url.set_path(&format!("batch-data/{}", name));
            url.set_query(Some(&query));
            Client::new()
                .get(url.as_ref())
                .send()
                .map_err(Into::into)
                .and_then(|response| {
                    let status_code = response.status();
                    if status_code == StatusCode::OK {
                        Ok(response)
                    } else {
                        Err(::std::io::Error::new(
                            ::std::io::ErrorKind::Other,
                            format!("http request for {} status code is {}", url, status_code),
                        )
                        .into())
                    }
                })
        })
    }
}
