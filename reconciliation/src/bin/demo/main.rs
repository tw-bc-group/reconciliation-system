#[macro_use]
extern crate log;

mod error;
mod job;

use std::path::Path;

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

    let tests_path = Path::new("tests");
    let job_manager = JobQueue::new(
        FileLoader::new(tests_path.join("mock_data")),
        tests_path.join("plugin"),
    );

    let job_queue = web::Data::new(job_manager);

    HttpServer::new(move || {
        App::new()
            .app_data(job_queue.clone())
            .wrap(Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/reconciliation").route(web::post().to(reconciliation)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
