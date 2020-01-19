use crate::job::prelude::*;
use actix_web::ResponseError;

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("lib reconciliation error, `{0}`")]
    Lib(#[from] anyhow::Error),
    #[error("send job error, `{0}`")]
    SendJob(#[from] crossbeam_channel::TrySendError<JobQueueMessage>),
    #[error("std's io error, `{0}`")]
    Stdio(#[from] std::io::Error),
}

pub(crate) type Result<T> = ::std::result::Result<T, Error>;

impl ResponseError for Error {}
