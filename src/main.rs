use actix_web::{
    http::Method, middleware::Logger, web, App, HttpResponse, HttpServer, Responder, Result,
};
use dotenvy::dotenv;
use redis::aio::ConnectionManager;
use std::env;

mod models {
    use redis::aio::ConnectionManager;

    #[derive(Clone)]
    pub struct AppState {
        pub redis: ConnectionManager,
    }
}

mod handlers {
    use actix_web::{Error, HttpRequest, HttpResponse};

    pub async fn healthcheck(_req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(HttpResponse::Ok()
            .content_type("text/plain")
            .body("Hello!".to_string()))
    }
}

async fn default_handler(_req_method: Method) -> Result<impl Responder> {
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("Hello!".to_string()))
}

use crate::models::AppState;
use handlers::healthcheck;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("PORT").expect("Missing port number");
    let port = port.parse::<u16>().expect("Invalid port given");

    let redis_url = env::var("REDIS_PRIVATE_URL").expect("Missing Redis URL");
    let client = redis::Client::open(redis_url).unwrap();
    let backend = ConnectionManager::new(client).await.unwrap();

    let data = web::Data::new(AppState { redis: backend });

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(data.clone())
            .service(web::resource("/healthcheck").route(web::get().to(healthcheck)))
            .default_service(web::to(default_handler))
    })
    .bind(("0.0.0.0", port))?
    .run();

    println!("Server running at http://0.0.0.0:{}/", port);

    server.await
}
