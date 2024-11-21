use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use crate::schema::students;
use validator::Validate;

#[derive(Debug, Queryable, Serialize)] // Add Debug trait here
pub struct Student {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub course: String,
}

#[derive(Debug, Insertable, Deserialize, Validate)] // Add Debug trait here
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
