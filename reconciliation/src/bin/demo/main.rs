mod job;

use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use crate::job::*;
use actix_web::{middleware::Logger, web, App, HttpServer};
use reconciliation::prelude::*;

async fn reconciliation<R, L>(job: web::Data<JobManager<R, L>>) -> &'static str
where
    R: Read,
    L: Loader<R> + Sync,
{
    job.run();
    "OK"
}

#[actix_rt::main]
async fn main() -> Result<(), ::std::io::Error> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let tests_path = Path::new("tests");
    let job_manager = JobManager::new(
        FileLoader::new(tests_path.join("mock_data")),
        tests_path.join("plugin"),
    );

    let job_manager = web::Data::new(job_manager);

    HttpServer::new(move || {
        App::new()
            .app_data(job_manager.clone())
            .wrap(Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(
                web::resource("/reconciliation")
                    .route(web::post().to(reconciliation::<File, FileLoader<PathBuf>>)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
