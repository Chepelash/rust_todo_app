#![allow(unused)]

use crate::{
    database::{model::todo_entry::TodoEntry, repository},
    AppState,
};
use actix_web::{delete, get, post, web, HttpResponse, Responder};

pub async fn get_all_records(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body("todo")
}

pub async fn create_new_record(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body("todo")
}

pub async fn delete_record_with_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> impl Responder {
    HttpResponse::Ok().body("todo")
}

pub async fn update_record_with_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> impl Responder {
    HttpResponse::Ok().body("todo")
}
