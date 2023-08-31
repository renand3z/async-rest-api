use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
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