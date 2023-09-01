use actix_web::{get, web, App, HttpServer};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
// use serde_json;
mod todolist;
use todolist::services;

struct AppState {
    todolist_entries: Mutex<Vec<TodolistEntry>>
}

#[derive(Serialize, Deserialize, Clone)]
struct TodolistEntry {
    id: i32,
    date: i64,
    title: String,
}

#[get("/")]
async fn index() -> String {
   return "working".to_string();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        todolist_entries: Mutex::new(vec![])
});

    HttpServer::new(move || {
        App::new()
        .app_data(app_data.clone())
        .service(index)
        .configure(services::config)
            .route("/", web::get().to(get_index))
            .route("/{id}", web::get().to(get_item)) 
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_index() -> String {
    "Hello World!".to_string()
}

async fn get_item(path: web::Path<u32>) -> String {
    format!("Item ID: {}", path) 
}