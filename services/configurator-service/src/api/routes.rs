use actix_web::web;
use crate::application::NetworkApplicationService;
use crate::api::handlers::network_handlers::{
    create_network, update_network, delete_network, get_network, list_networks,
};

pub fn configure_routes(cfg: &mut web::ServiceConfig, service: web::Data<NetworkApplicationService>) {
    cfg.service(
        web::scope("/api/v1/networks")
            .app_data(service.clone())
            .route("", web::post().to(create_network))
            .route("", web::get().to(list_networks))
            .route("/{id}", web::get().to(get_network))
            .route("/{id}", web::put().to(update_network))
            .route("/{id}", web::delete().to(delete_network)),
    );
}
