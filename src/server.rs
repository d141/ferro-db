use actix_web::{web, Responder, HttpResponse};
use std::sync::{Arc, Mutex};
use super::FerroDB;


pub async fn set_fn(
    db: web::Data<Arc<Mutex<FerroDB>>>,
    path: web::Path<(String, String)>,
    body: String
) -> impl Responder {
    let (collection, key) = path.into_inner();
    let mut db = db.lock().unwrap();
    match db.set(&collection, key, body) {
        Ok(_) => HttpResponse::Ok().body("Value set successfully"),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn get_fn(
    db: web::Data<Arc<Mutex<FerroDB>>>,
    path: web::Path<(String, String)>
) -> impl Responder {
    let (collection, key) = path.into_inner();
    let db = db.lock().unwrap();
    match db.get(&collection, &key) {
        Ok(Some(value)) => HttpResponse::Ok().body(value),
        Ok(None) => HttpResponse::NotFound().body("Key not found"),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn unset_fn(
    db: web::Data<Arc<Mutex<FerroDB>>>,
    path: web::Path<(String, String)>
) -> impl Responder {
    let (collection, key) = path.into_inner();
    let mut db = db.lock().unwrap();
    match db.unset(&collection, &key) {
        Ok(Some(value)) => HttpResponse::Ok().body(format!("Removed: {}", value)),
        Ok(None) => HttpResponse::NotFound().body("Key not found"),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}