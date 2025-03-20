#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware};
use env_logger::Env;
use actix_web::dev::Service;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use serde_json::json;
use validator::Validate;
use std::time::Duration;

mod schema;
mod models;

use models::{NewStudent, Student};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    r2d2::Pool::builder()
        .max_size(5)
        .connection_timeout(Duration::from_secs(3))
        .build(manager)
        .expect("Failed to create pool")
}

async fn create_student(
    pool: web::Data<DbPool>,
    new_student: web::Json<NewStudent>
) -> impl Responder {
    log::info!("Received request to create a student: {:?}", new_student);

    let new_student = new_student.into_inner();

    if let Err(errors) = new_student.validate() {
        log::error!("Validation errors: {:?}", errors);
        return HttpResponse::BadRequest().json(errors);
    }

    let conn = pool.get().map_err(|e| {
        log::error!("Failed to get database connection: {:?}", e);
        HttpResponse::InternalServerError().json(json!({"error": "Database connection error"}))
    })?;

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
    // Initialize logging with a more detailed format
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Create database connection pool
    let pool = establish_connection_pool();
    log::info!("Database connection pool established");

    HttpServer::new(move || {
        log::info!("Setting up Actix application...");
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Compress::default())
            .wrap_fn(|req, srv| {
                let fut = srv.call(req);
                async move {
                    let res = fut.await?;
                    Ok(res)
                }
            })
            .route("/students", web::post().to(create_student))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
