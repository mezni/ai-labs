use actix_web::{web, HttpResponse, Responder};
use crate::application::services::network_service::NetworkService;
use crate::application::dtos::network_dtos::{CreateNetworkDto, UpdateNetworkDto};
use std::sync::Arc;

pub fn init_handlers(cfg: &mut web::ServiceConfig, service: Arc<NetworkService<impl crate::domain::network::NetworkRepository>>) {
    cfg.service(
        web::scope("/networks")
            .route("", web::post().to(create_network::<_>))
            .route("/{id}", web::put().to(update_network::<_>))
            .route("/{id}", web::get().to(get_network::<_>))
            .route("", web::get().to(get_all_networks::<_>))
            .route("/{id}", web::delete().to(delete_network::<_>))
    );
}

async fn create_network<R: crate::domain::network::NetworkRepository>(
    service: web::Data<Arc<NetworkService<R>>>,
    dto: web::Json<CreateNetworkDto>
) -> impl Responder {
    match service.create_network(dto.into_inner()).await {
        Ok(network) => HttpResponse::Ok().json(network),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn update_network<R: crate::domain::network::NetworkRepository>(
    service: web::Data<Arc<NetworkService<R>>>,
    path: web::Path<i32>,
    dto: web::Json<UpdateNetworkDto>
) -> impl Responder {
    match service.update_network(path.into_inner(), dto.into_inner()).await {
        Ok(network) => HttpResponse::Ok().json(network),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn get_network<R: crate::domain::network::NetworkRepository>(
    service: web::Data<Arc<NetworkService<R>>>,
    path: web::Path<i32>
) -> impl Responder {
    match service.get_network(path.into_inner()).await {
        Ok(Some(network)) => HttpResponse::Ok().json(network),
        Ok(None) => HttpResponse::NotFound().body("Network not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn get_all_networks<R: crate::domain::network::NetworkRepository>(
    service: web::Data<Arc<NetworkService<R>>>
) -> impl Responder {
    match service.get_all_networks().await {
        Ok(networks) => HttpResponse::Ok().json(networks),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn delete_network<R: crate::domain::network::NetworkRepository>(
    service: web::Data<Arc<NetworkService<R>>>,
    path: web::Path<i32>
) -> impl Responder {
    match service.delete_network(path.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("Deleted successfully"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
