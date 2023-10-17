use actix_web::{web, App, HttpServer};

mod database;
mod rest_controller;

use database::repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let db = repository::DbOperatorNew::new("localhost:8000");
    let db_data = web::Data::new(db.connect().await.expect("Cannot connect to DB"));

    HttpServer::new(move || {
        App::new().app_data(db_data.clone()).service(
            web::scope("/todo")
                .route(
                    "/get_all_records",
                    web::get().to(rest_controller::controller::get_all_records),
                )
                .route(
                    "/create_new_record",
                    web::get().to(rest_controller::controller::create_new_record),
                )
                .route(
                    "/delete/{id}",
                    web::delete().to(rest_controller::controller::delete_record_with_id),
                )
                .route(
                    "/update",
                    web::post().to(rest_controller::controller::update_record),
                ),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
