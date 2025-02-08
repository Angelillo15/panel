use std::sync::Arc;
use actix_web::{get, post, web, HttpResponse, Responder, Scope};
use crate::auth::service::auth::{AuthService, LoginRequest};

#[post("/login")]
async fn login(login_service: web::Data<Arc<dyn AuthService>>, request: web::Json<LoginRequest>) -> impl Responder {
    println!("{:?}", request);
    request.username.clone();
    HttpResponse::Ok()
}

#[get("/")]
async fn index() -> impl Responder {
    "Buenos dias"
}

pub fn endpoints(scope: Scope) -> Scope {
    scope.service(login).service(index)
}