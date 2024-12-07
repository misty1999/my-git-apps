use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use log::{info, error};
use serde::Deserialize;
use actix_cors::Cors;
use serde_json::json;

mod create_issue;
mod config;

#[derive(Deserialize)]
struct Message {
    title: String,
    body: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    info!("Received request to /");
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/issue")]
async fn issue(message: web::Json<Message>) -> impl Responder {
    info!("Received message - title: {}, body: {}", message.title, message.body);
    
    match create_issue::create_issue(&message.title, &message.body).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": format!("Branch for issue '{}' created", message.title)
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to create branch: {}", e)
        }))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    // 明示的に標準出力に設定
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .target(env_logger::Target::Stdout)
        .init();

    info!("Starting server at http://0.0.0.0:8080");
    
    let server = HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Content-Type"]);

        App::new()
            .wrap(cors)
            .service(hello)
            .service(issue)
    })
    .bind(("0.0.0.0", 8080))?;

    info!("Server successfully bound to port 8080");
    
    match server.run().await {
        Ok(_) => {
            info!("Server shutdown successfully");
            Ok(())
        },
        Err(e) => {
            error!("Server error: {}", e);
            Err(e)
        }
    }
}
