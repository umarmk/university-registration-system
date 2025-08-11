use actix_web::error::ErrorUnauthorized;
use actix_web::http::header;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::future::{ready, Ready};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,  // Subject (user ID)
    exp: usize,   // Expiration time
    role: String, // User role
    #[serde(skip_serializing_if = "Option::is_none")]
    aud: Option<String>, // Optional audience
}

pub struct JwtAuth;

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddleware { service }))
    }
}

pub struct JwtAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Get the JWT secret key from environment
        let jwt_secret = match env::var("JWT_SECRET") {
            Ok(s) => s,
            Err(_) => {
                return Box::pin(async { Err(ErrorUnauthorized("Server configuration error")) });
            }
        };

        // Get the Authorization header
        let auth_header = match req.headers().get(header::AUTHORIZATION) {
            Some(h) => h,
            None => {
                return Box::pin(async { Err(ErrorUnauthorized("No authorization header found")) });
            }
        };

        // Extract the token
        let auth_str = match auth_header.to_str() {
            Ok(s) => s,
            Err(_) => {
                return Box::pin(async { Err(ErrorUnauthorized("Invalid authorization header")) });
            }
        };

        if !auth_str.starts_with("Bearer ") {
            return Box::pin(async { Err(ErrorUnauthorized("Invalid authorization scheme")) });
        }

        let token = auth_str[7..].trim();
        if token.is_empty() {
            return Box::pin(async { Err(ErrorUnauthorized("Empty token")) });
        }

        // Validate the token
        let validation = Validation::new(Algorithm::HS256);
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &validation,
        ) {
            Ok(token_data) => {
                // Set claims info in request extensions
                req.extensions_mut().insert(token_data.claims);
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            Err(_) => Box::pin(async { Err(ErrorUnauthorized("Invalid token")) }),
        }
    }
}
