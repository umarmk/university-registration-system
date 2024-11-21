#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use env_logger::Env;
use actix_web::dev::Service; 
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use serde_json::json;
use validator::Validate;

mod schema;
mod models;

use models::{NewStudent, Student};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    log::info!("Connecting to database at {}", database_url); // Log database connection
    PgConnection::establish(&database_url).expect("Error connecting to database")
}

async fn create_student(new_student: web::Json<NewStudent>) -> impl Responder {
    log::info!("Received request to create a student: {:?}", new_student);

    let new_student = new_student.into_inner();

    if let Err(errors) = new_student.validate() {
        log::error!("Validation errors: {:?}", errors);
        return HttpResponse::BadRequest().json(errors);
    }

    log::info!("Establishing database connection...");
    let mut conn = establish_connection();

    log::info!("Attempting to insert student into the database...");
    let result = diesel::insert_into(schema::students::dsl::students)
        .values(&new_student)
        .get_result::<Student>(&mut conn);

    match result {
        Ok(student) => {
            log::info!("Successfully inserted student: {:?}", student);
            HttpResponse::Created().json(student)
        }
        Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => {
            log::error!("Duplicate entry for email: {:?}", new_student.email);
            HttpResponse::Conflict().json(json!({"error": "Email already exists"}))
        }
        Err(e) => {
            log::error!("Database insertion error: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"error": "Failed to create student"}))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        log::info!("Setting up Actix application...");
        App::new()
            .wrap_fn(|req, srv| {
                log::info!("Incoming request: {:?}", req);
                srv.call(req)
            })
            .route("/students", web::post().to(create_student))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
