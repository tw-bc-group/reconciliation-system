use crate::job::prelude::*;
use actix_web::ResponseError;

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("lib reconciliation error, `{0}`")]
    Lib(#[from] anyhow::Error),
    #[error("text nonce error, `{0}`")]
    TextNonce(String),
    #[error("send job error, `{0}`")]
    SendJob(#[from] crossbeam_channel::TrySendError<JobQueueMessage>),
}

pub(crate) type Result<T> = ::std::result::Result<T, Error>;

impl ResponseError for Error {}
