use std::env::set_var;
use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use actix_web::web::Data;
use env_logger::Env;
use sea_orm::{Database, DatabaseConnection};
use panel::auth;
use panel::auth::infrastructure::sql::auth_impl::SQLImpl;
use panel::auth::service::auth::AuthService;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    set_var("RUST_LOG", "debug");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let db: DatabaseConnection = Database::connect("postgres://panel:1234@localhost/panel?currentSchema=my_schema")
        .await
        .expect("database connection");

    let auth_service: Data<Arc<dyn AuthService>> = Data::new(Arc::new(SQLImpl::new(db.clone())));

    HttpServer::new(move || {
        let mut app = App::new();
        app = app.app_data(auth_service.clone());

        app = app.service(auth::controller::login::endpoints(web::scope("/auth")));
        app
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}