use crate::service::patient_service::PatientService;
use crate::{model::patient::Patient, service::generic_service::GenericService};
use actix_web::{web, HttpResponse, Responder};

pub async fn get_patient(
    service: web::Data<PatientService>,
    id: web::Path<String>,
) -> impl Responder {
    match service.handle_get(&id).await {
        Ok(patient) => HttpResponse::Ok().json(patient),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn create_patient(
    service: web::Data<PatientService>,
    patient: web::Json<Patient>,
) -> impl Responder {
    match service.handle_create(patient.into_inner()).await {
        Ok(patient) => HttpResponse::Created().json(patient),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_patient(
    service: web::Data<PatientService>,
    patient: web::Json<Patient>,
) -> impl Responder {
    match service.handle_update(patient.into_inner()).await {
        Ok(patient) => HttpResponse::Ok().json(patient),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_patient(
    service: web::Data<PatientService>,
    id: web::Path<String>,
) -> impl Responder {
    match service.handle_delete(&id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn list_patients(service: web::Data<PatientService>) -> impl Responder {
    match service.handle_list().await {
        Ok(patients) => HttpResponse::Ok().json(patients),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
