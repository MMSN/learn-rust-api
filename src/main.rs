use actix_web::{web, App, HttpServer, Responder, get, middleware::Logger};

mod utils;
mod routes;


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello, {}!", name)
}

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
            .service(greet)
    })
    .bind(format!("{}:{}", address, port))?
    .run()
    .await
}
