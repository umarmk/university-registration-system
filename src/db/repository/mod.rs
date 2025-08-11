use async_trait::async_trait;
use diesel::prelude::*;
use crate::models::{User, Role, Student, AuditLog};
use crate::db::error::DbError;

#[async_trait]
pub trait Repository {
    async fn find_user_by_id(&self, id: i32) -> Result<Option<User>, DbError>;
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, DbError>;
    async fn create_user(&self, new_user: &NewUser) -> Result<User, DbError>;
    async fn update_user(&self, id: i32, user: &UpdateUser) -> Result<User, DbError>;
    async fn delete_user(&self, id: i32) -> Result<bool, DbError>;
    
    async fn find_role_by_id(&self, id: i32) -> Result<Option<Role>, DbError>;
    async fn find_role_by_name(&self, name: &str) -> Result<Option<Role>, DbError>;
    
    async fn find_student_by_id(&self, id: i32) -> Result<Option<Student>, DbError>;
    async fn create_student(&self, new_student: &NewStudent) -> Result<Student, DbError>;
    async fn update_student(&self, id: i32, student: &UpdateStudent) -> Result<Student, DbError>;
    async fn delete_student(&self, id: i32) -> Result<bool, DbError>;
    async fn list_students(&self, limit: i64, offset: i64) -> Result<Vec<Student>, DbError>;
    
    async fn create_audit_log(&self, log: &NewAuditLog) -> Result<AuditLog, DbError>;
    async fn list_audit_logs(&self, limit: i64, offset: i64) -> Result<Vec<AuditLog>, DbError>;
}

pub mod diesel_repository;
pub use diesel_repository::DieselRepository; 