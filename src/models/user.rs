use crate::models::role::Role;
use crate::schema::{user_tokens, users};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Role))]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role_id: i32,
    pub is_active: bool,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    #[validate(length(min = 3, max = 100))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // user id
    pub exp: usize,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = user_tokens)]
pub struct UserToken {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        hash(password.as_bytes(), DEFAULT_COST)
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(password.as_bytes(), &self.password_hash).unwrap_or(false)
    }

    pub fn generate_token(
        &self,
        secret: &str,
        role_name: &str,
        expiration: usize,
    ) -> Result<String, JwtError> {
        let claims = Claims {
            sub: self.id,
            exp: expiration,
            role: role_name.to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
    }

    pub fn verify_token(token: &str, secret: &str) -> Result<Claims, JwtError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }
}
