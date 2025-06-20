use actix_cors::Cors;
use actix_web::{App, HttpServer, Responder, middleware::Logger, web};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        // CORS configuration
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .expose_headers(&[actix_web::http::header::CONTENT_DISPOSITION])
            .supports_credentials()
            .max_age(3600);

        App::new()
           
            .wrap(Logger::default())
            .wrap(cors)
        
            .service(web::resource("/health").to(|| async { "Server is running" }))
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
