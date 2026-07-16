use actix_web::{ App, HttpServer, middleware::Logger, web };
use jsonwebtoken::crypto::rust_crypto::DEFAULT_PROVIDER;
use migration::{ Migrator, MigratorTrait };
use sea_orm::{ Database, DatabaseConnection };
use utils::app_state::AppState;

mod utils;
mod routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }

    dotenv::dotenv().ok();

    env_logger::init();

    DEFAULT_PROVIDER
        .install_default()
        .expect("Failed to install jsonwebtoken crypto provider");


    let address: String = (*utils::constants::ADDRESS).clone();
    let port: u16 = (*utils::constants::PORT).clone();
    let database_url: String = (*utils::constants::DATABASE_URL).clone();

    let db: DatabaseConnection = Database::connect(
        database_url
    )
    .await
    .unwrap();

    Migrator::up(&db, None).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new( AppState { db : db.clone() } ))
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
            .configure(routes::auth_routes::config)
    })
    .bind(format!("{}:{}", address, port))?
    .run()
    .await
}
