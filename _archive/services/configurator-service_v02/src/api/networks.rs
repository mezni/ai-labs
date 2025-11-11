// src/api/networks.rs
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use crate::application::dtos::network_dtos::{CreateNetworkDto, UpdateNetworkDto, NetworkDto};
use crate::application::services::network_application_service::NetworkApplicationService;
use utoipa::ToSchema;

pub type NetworkService<R> = NetworkApplicationService<R>;

#[post("/networks")]
pub async fn create_network_handler<R: crate::domain::repositories::network_repository::NetworkRepository>(
    service: web::Data<NetworkService<R>>,
    payload: web::Json<CreateNetworkDto>,
) -> impl Responder {
    match service.create_network(payload.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[get("/networks/{id}")]
pub async fn get_network_by_id_handler<R: crate::domain::repositories::network_repository::NetworkRepository>(
    service: web::Data<NetworkService<R>>,
    id: web::Path<i32>,
) -> impl Responder {
    match service.get_by_id(id.into_inner()).await {
        Ok(Some(network)) => HttpResponse::Ok().json(network),
        Ok(None) => HttpResponse::NotFound().body("Network not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/networks")]
pub async fn get_all_networks_handler<R: crate::domain::repositories::network_repository::NetworkRepository>(
    service: web::Data<NetworkService<R>>,
) -> impl Responder {
    match service.get_all_networks().await {
        Ok(networks) => HttpResponse::Ok().json(networks),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/networks/{id}")]
pub async fn update_network_handler<R: crate::domain::repositories::network_repository::NetworkRepository>(
    service: web::Data<NetworkService<R>>,
    id: web::Path<i32>,
    payload: web::Json<UpdateNetworkDto>,
) -> impl Responder {
    match service.update_network(id.into_inner(), payload.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[delete("/networks/{id}")]
pub async fn delete_network_handler<R: crate::domain::repositories::network_repository::NetworkRepository>(
    service: web::Data<NetworkService<R>>,
    id: web::Path<i32>,
) -> impl Responder {
    match service.delete_network(id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}
