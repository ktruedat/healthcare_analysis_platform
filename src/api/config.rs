use crate::api::patient::{
    create_patient, delete_patient, get_patient, list_patients, update_patient,
};
use actix_web::{web, Responder};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/patients")
            .route("", web::get().to(list_patients))
            .route("", web::post().to(create_patient))
            .route("/{id}", web::get().to(get_patient))
            .route("/{id}", web::put().to(update_patient))
            .route("/{id}", web::delete().to(delete_patient)),
    );
    cfg.service(web::resource("/hello").to(hello_from_server));
}

async fn hello_from_server() -> impl Responder {
    "Hello from server"
}
