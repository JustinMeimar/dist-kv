use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::Mutex;
use std::collections::HashMap;

struct AppState {
    store: Mutex<HashMap<String, String>>,
}

async fn get(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let store = data.store.lock().unwrap();
    match store.get(path.as_str()) {
        Some(value) => HttpResponse::Ok().body(value.to_string()),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn put(data: web::Data<AppState>, path: web::Path<String>, body: web::Bytes) -> impl Responder {
    let mut store = data.store.lock().unwrap();
    let key = path.into_inner();
    let value = String::from_utf8(body.to_vec()).unwrap();

    if store.contains_key(&key) {
        HttpResponse::Forbidden().finish()
    } else {
        store.insert(key, value);
        HttpResponse::Ok().finish()
    }
}

async fn delete(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let mut store = data.store.lock().unwrap();
    store.remove(path.as_str());
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        store: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/{key}", web::get().to(get))
            .route("/{key}", web::put().to(put))
            .route("/{key}", web::delete().to(delete))
    })
    .bind("localhost:3000")?
    .run()
    .await
}