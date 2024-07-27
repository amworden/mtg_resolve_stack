// src/main.rs

use actix_web::{web, App, HttpServer, HttpResponse};
use serde_json::json;
use std::sync::Mutex;

mod models;
use models::{Card, Stack, ResolutionResult};

struct AppState {
    stack: Mutex<Stack>,
}

async fn add_card_to_stack(card: web::Json<Card>, data: web::Data<AppState>) -> HttpResponse {
    let mut stack = data.stack.lock().unwrap();
    stack.add_to_stack(card.into_inner());
    HttpResponse::Ok().json(json!({"status": "card added to stack"}))
}

async fn resolve_stack(data: web::Data<AppState>) -> HttpResponse {
    let mut stack = data.stack.lock().unwrap();
    let resolved_cards = stack.resolve_stack();
    HttpResponse::Ok().json(resolved_cards)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let stack = web::Data::new(AppState {
        stack: Mutex::new(Stack::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(stack.clone())
            .route("/add_card", web::post().to(add_card_to_stack))
            .route("/resolve_stack", web::get().to(resolve_stack))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
