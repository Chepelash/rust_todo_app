use crate::database::repository::DbOperatorConnected;
use crate::model::todo_entry::TodoEntry;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_all_records(data: web::Data<DbOperatorConnected>) -> impl Responder {
    let request_result = data.get_all_records().await;
    match request_result {
        Ok(result) => HttpResponse::Ok().body(
            result
                .iter()
                .map(|record| serde_json::to_string(record).expect("cannot fail"))
                .collect::<Vec<String>>()
                .join(","),
        ),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn create_new_record(data: web::Data<DbOperatorConnected>) -> impl Responder {
    let request_result = data.create_record(&TodoEntry::default()).await;
    match request_result {
        Ok(()) => HttpResponse::Ok().body("created"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn delete_record_with_id(
    data: web::Data<DbOperatorConnected>,
    id: web::Path<String>,
) -> impl Responder {
    let request_result = data.delete_record(&id).await;
    match request_result {
        Ok(()) => HttpResponse::Ok().body("deleted"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn update_record(
    data: web::Data<DbOperatorConnected>,
    record: web::Json<TodoEntry>,
) -> impl Responder {
    let request_result = data.update_record(&record).await;
    match request_result {
        Ok(()) => HttpResponse::Ok().body("updated"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
