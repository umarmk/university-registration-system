use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::schema::roles;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = roles)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, Insertable)]
#[diesel(table_name = roles)]
pub struct NewRole {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    pub description: Option<String>,
}

impl Role {
    pub fn is_admin(&self) -> bool {
        self.name == "admin"
    }

    pub fn is_user(&self) -> bool {
        self.name == "user"
    }
} 