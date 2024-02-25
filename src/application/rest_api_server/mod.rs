use std::thread;
use actix_web::{App, HttpServer, Responder, web};

async fn handle_get_request(path: web::Path<(String,)>) -> impl Responder {
    return format!("TEST RESPONSE: {}", path.0);
}

pub fn spawn_rest_server() {
    thread::spawn(|| {
        let _ = run_rest_server();
    });
}

#[actix_web::main]
async fn run_rest_server() -> std::io::Result<()> {
    return HttpServer::new(|| {
                App::new()
                    .route("/{data_source}", web::get().to(handle_get_request))
            })
            .bind("127.0.0.1:8080").expect("Failed to bind address for REST API server!")
            .run()
            .await;
}