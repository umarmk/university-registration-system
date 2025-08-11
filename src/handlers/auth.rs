use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::models::role::Role;
use crate::models::user::{NewUser, User};
use crate::schema::{roles, users};
use crate::DbPool;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 100))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role_id: i32,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    #[serde(rename = "accessToken")]
    pub access_token: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role_id: i32,
    pub is_active: bool,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            role_id: user.role_id,
            is_active: user.is_active,
        }
    }
}

pub async fn login(pool: web::Data<DbPool>, login_req: web::Json<LoginRequest>) -> HttpResponse {
    let login_req = login_req.into_inner();

    if let Err(errors) = login_req.validate() {
        log::error!("Login validation errors: {:?}", errors);
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

    let email = login_req.email.clone();
    let result = web::block(move || {
        // Find user by email
        let user = users::table
            .filter(users::email.eq(&login_req.email))
            .filter(users::is_active.eq(true))
            .first::<User>(&mut *conn)
            .optional()?;

        if let Some(user) = user {
            // Verify password
            if user.verify_password(&login_req.password) {
                // Get user role
                let role = roles::table.find(user.role_id).first::<Role>(&mut *conn)?;

                // Generate JWT token
                let secret =
                    std::env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());
                let expiration = (Utc::now() + Duration::hours(24)).timestamp() as usize;

                let token = user
                    .generate_token(&secret, &role.name, expiration)
                    .map_err(|e| {
                        diesel::result::Error::DatabaseError(
                            diesel::result::DatabaseErrorKind::Unknown,
                            Box::new(format!("Token generation failed: {}", e)),
                        )
                    })?;

                // Update last login
                diesel::update(users::table.find(user.id))
                    .set(users::last_login.eq(Some(Utc::now())))
                    .execute(&mut *conn)?;

                Ok(AuthResponse {
                    user: UserResponse::from(user),
                    access_token: token,
                    role: role.name,
                })
            } else {
                Err(diesel::result::Error::NotFound)
            }
        } else {
            Err(diesel::result::Error::NotFound)
        }
    })
    .await;

    match result {
        Ok(db_result) => match db_result {
            Ok(auth_response) => {
                log::info!("User logged in successfully: {}", auth_response.user.email);
                HttpResponse::Ok().json(auth_response)
            }
            Err(diesel::result::Error::NotFound) => {
                log::warn!("Login attempt with invalid credentials: {}", email);
                HttpResponse::Unauthorized().json(json!({
                    "status": "error",
                    "message": "Invalid email or password"
                }))
            }
            Err(db_err) => {
                log::error!("Database error during login: {:?}", db_err);
                HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": "Authentication failed"
                }))
            }
        },
        Err(blocking_err) => {
            log::error!("Blocking error during login: {:?}", blocking_err);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Internal server error"
            }))
        }
    }
}

pub async fn register(
    pool: web::Data<DbPool>,
    register_req: web::Json<RegisterRequest>,
) -> HttpResponse {
    let register_req = register_req.into_inner();
    log::info!(
        "Registration request received for email: {}",
        register_req.email
    );

    if let Err(errors) = register_req.validate() {
        log::error!("Registration validation errors: {:?}", errors);
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
        // Check if user already exists
        let existing_user = users::table
            .filter(
                users::email
                    .eq(&register_req.email)
                    .or(users::username.eq(&register_req.username)),
            )
            .first::<User>(&mut *conn)
            .optional()?;

        if existing_user.is_some() {
            return Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                Box::new("User already exists".to_string()),
            ));
        }

        // Hash password
        let password_hash = User::hash_password(&register_req.password).map_err(|e| {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new(format!("Password hashing failed: {}", e)),
            )
        })?;

        // Create new user
        let new_user = NewUser {
            username: register_req.username,
            email: register_req.email,
            password_hash: password_hash,
            first_name: register_req.first_name,
            last_name: register_req.last_name,
            role_id: register_req.role_id,
        };

        let user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&mut *conn)?;

        Ok(UserResponse::from(user))
    })
    .await;

    match result {
        Ok(db_result) => match db_result {
            Ok(user_response) => {
                log::info!("User registered successfully: {}", user_response.email);
                HttpResponse::Created().json(json!({
                    "status": "success",
                    "message": "User registered successfully",
                    "user": user_response
                }))
            }
            Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            )) => HttpResponse::Conflict().json(json!({
                "status": "error",
                "message": "User with this email or username already exists"
            })),
            Err(db_err) => {
                log::error!("Database error during registration: {:?}", db_err);
                HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": "Failed to register user"
                }))
            }
        },
        Err(blocking_err) => {
            log::error!("Blocking error during registration: {:?}", blocking_err);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Internal server error"
            }))
        }
    }
}
