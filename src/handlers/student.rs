use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::models::{NewStudent, Student};
use crate::schema;
use crate::DbPool;

#[derive(Debug, Deserialize)]
pub struct StudentQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

pub async fn get_students(
    pool: web::Data<DbPool>,
    query: web::Query<StudentQuery>,
) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Failed to get database connection: {:?}", e);
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Database connection error"
            }));
        }
    };

    let result = web::block(move || {
        schema::students::dsl::students
            .limit(limit)
            .offset(offset)
            .load::<Student>(&mut *conn)
    })
    .await;

    match result {
        Ok(db_result) => match db_result {
            Ok(students) => {
                log::info!("Successfully fetched {} students", students.len());
                HttpResponse::Ok().json(json!({
                    "status": "success",
                    "data": students
                }))
            }
            Err(db_err) => {
                log::error!("Database error fetching students: {:?}", db_err);
                HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": "Failed to fetch students"
                }))
            }
        },
        Err(blocking_err) => {
            log::error!("Blocking error: {:?}", blocking_err);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Internal server error"
            }))
        }
    }
}

pub async fn create_student(
    pool: web::Data<DbPool>,
    new_student: web::Json<NewStudent>,
) -> HttpResponse {
    let new_student = new_student.into_inner();

    if let Err(errors) = new_student.validate() {
        log::error!("Validation errors: {:?}", errors);
        return HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "Validation failed",
            "errors": errors
        }));
    }

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Failed to get database connection: {:?}", e);
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Database connection error"
            }));
        }
    };

    let result = web::block(move || {
        diesel::insert_into(schema::students::dsl::students)
            .values(&new_student)
            .get_result::<Student>(&mut *conn)
    })
    .await;

    match result {
        Ok(db_result) => match db_result {
            Ok(student) => {
                log::info!("Successfully created student: {:?}", student);
                HttpResponse::Created().json(json!({
                    "status": "success",
                    "data": student
                }))
            }
            Err(db_err) => {
                log::error!("Database error creating student: {:?}", db_err);
                HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": "Failed to create student"
                }))
            }
        },
        Err(blocking_err) => {
            log::error!("Blocking error: {:?}", blocking_err);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Internal server error"
            }))
        }
    }
}
