use std::sync::Arc;
use actix_web::{post, web, HttpResponse, Responder, Scope};
use crate::actix::validated_json::ValidatedJson;
use crate::auth::request::login::LoginRequest;
use crate::auth::request::register::RegisterRequest;
use crate::auth::service::auth::{AuthService};

#[post("/login")]
async fn login(login_service: web::Data<Arc<dyn AuthService>>, request: ValidatedJson<LoginRequest>) -> impl Responder {
    match login_service.login(&request).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::BadRequest().json(error),
    }
}

#[post("/register")]
async fn register(login_service: web::Data<Arc<dyn AuthService>>, request: ValidatedJson<RegisterRequest>) -> impl Responder {
    match login_service.register(&request).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::BadRequest().json(error),
    }
}
pub fn endpoints(scope: Scope) -> Scope {
    scope.service(login).service(register)
}