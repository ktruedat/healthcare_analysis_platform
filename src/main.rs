use actix_web::{web, App, HttpServer};
use tracing::error;

mod api;
mod cfg;
mod db;
mod model;
mod repository;
mod service;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    if let Err(e) = run().await {
        error!("Error: {:?}", e);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = cfg::parse::parse_yaml_config("config.yaml")?;
    let db = db::surreal::connect(&config).await?;

    let repo = repository::repo::Repository::new(db, "patients");
    let patient_service = service::patient_service::PatientService::new(repo);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(patient_service.clone()))
            .configure(api::config::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
