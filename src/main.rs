use actix_web::{ App, HttpServer, middleware::Logger };

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

    let address: String = (*utils::constants::ADDRESS).clone();

    let port: u16 = (*utils::constants::PORT).clone();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
    })
    .bind(format!("{}:{}", address, port))?
    .run()
    .await
}
