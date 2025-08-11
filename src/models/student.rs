use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use crate::schema::students;
use validator::Validate;
use chrono::{DateTime, Utc};

#[derive(Debug, Queryable, Serialize)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub course: String,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize, Validate)]
#[diesel(table_name = students)]
pub struct NewStudent {
    #[validate(length(min = 2))]
    pub name: String,
    
    #[validate(length(min = 10, max = 15))]
    pub phone: String,
    
    #[validate(email)]
    pub email: String,
    
    pub course: String,
} 