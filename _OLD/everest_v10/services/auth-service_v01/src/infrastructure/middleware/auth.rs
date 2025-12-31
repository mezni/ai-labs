use crate::core::errors::AppError;
use crate::infrastructure::keycloak_client::KeycloakClient;
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::sync::Arc;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let keycloak = req
        .app_data::<actix_web::web::Data<Arc<dyn KeycloakClient>>>()
        .expect("Keycloak client not found");

    match keycloak.verify_token(credentials.token()).await {
        Ok(token_data) => {
            req.extensions_mut().insert(token_data);
            Ok(req)
        }
        Err(_) => Err((AppError::Unauthorized("Invalid token".into()).into(), req)),
    }
}

pub async fn admin_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let keycloak = req
        .app_data::<actix_web::web::Data<Arc<dyn KeycloakClient>>>()
        .expect("Keycloak client not found");

    match keycloak.verify_token(credentials.token()).await {
        Ok(token_data) => {
            let roles = token_data["realm_access"]["roles"]
                .as_array()
                .and_then(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .collect::<Vec<&str>>()
                        .into()
                });

            if let Some(roles) = roles {
                if roles.contains(&"admin") {
                    req.extensions_mut().insert(token_data);
                    return Ok(req);
                }
            }

            Err((
                AppError::Unauthorized("Admin access required".into()).into(),
                req,
            ))
        }
        Err(_) => Err((AppError::Unauthorized("Invalid token".into()).into(), req)),
    }
}
