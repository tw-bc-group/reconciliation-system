#[macro_use]
extern crate log;

mod error;
mod job;

use std::env;

use crate::{error::*, job::prelude::*};
use actix_web::{middleware::Logger, web, App, HttpServer};
use reconciliation::prelude::*;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct ReconciliationTime {
    start: i64,
    end: i64,
}

async fn reconciliation(
    time: web::Json<ReconciliationTime>,
    queue: web::Data<JobQueue>,
) -> Result<web::Json<Value>> {
    queue
        .new_job(time.start, time.end)
        .map(|id| web::Json(json!({ "id": id })))
}

#[actix_rt::main]
async fn main() -> ::std::result::Result<(), ::std::io::Error> {
    env_logger::init();

    let bind_address = format!(
        "{}:{}",
        env::var("HTTP_ADDRESS").unwrap_or_else(|_| String::from("127.0.0.1")),
        env::var("HTTP_PORT").unwrap_or_else(|_| String::from("8080"))
    );

    let job_queue = web::Data::new(JobQueue::new(
        HttpLoader::new(&env::var("HTTP_LOADER_URL").expect("env HTTP_LOADER_URL must be set"))
            .map_err(|err| ::std::io::Error::new(::std::io::ErrorKind::Other, err))?,
        "plugin",
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(job_queue.clone())
            .wrap(Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/reconciliation").route(web::post().to(reconciliation)))
    })
    .bind(&bind_address)?
    .run()
    .await
}
