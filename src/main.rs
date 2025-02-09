use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use directories::ProjectDirs;
use env_logger::Env;
use log::debug;
use panel::auth;
use panel::auth::infrastructure::sql::auth_impl::SQLImpl;
use panel::auth::service::auth::AuthService;
use panel::auth::service::hash_service::HashService;
use sea_orm::{Database, DatabaseConnection};
use std::env::set_var;
use std::sync::Arc;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = default_config_path())]
    config_path: String,
    #[arg(long)]
    host: Option<String>,
    #[arg(short, long)]
    port: Option<u16>,
    #[arg(long, short, action)]
    debug: bool,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.debug {
        set_var("RUST_LOG", "debug");
    }

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    debug!("Debug mode has been enabled!");

    let db: DatabaseConnection = Database::connect("mysql://root@localhost/panel")
        .await
        .expect("database connection");

    // TODO: change this in future
    let hash_service = Arc::new(HashService::new("todotochangethisinthefuture"));

    let auth_service: Data<Arc<dyn AuthService>> =
        Data::new(Arc::new(SQLImpl::new(db.clone(), hash_service.clone())));

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

fn default_config_path() -> String {
    match ProjectDirs::from("es", "angelillo15", "panel") {
        Some(dir) => format!("{}/config.toml", dir.config_dir().to_str().unwrap()).to_string(),
        None => "./config.toml".to_string(),
    }
}
