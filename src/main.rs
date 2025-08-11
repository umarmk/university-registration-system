extern crate diesel;
extern crate dotenv;

use actix_cors::Cors;
use actix_request_identifier::RequestIdentifier;
use actix_web::{middleware, web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use env_logger::Env;
use std::env;
use std::time::Duration;

mod handlers;
mod models;
mod schema;

use handlers::{auth, student};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .max_size(5)
        .min_idle(Some(1))
        .connection_timeout(Duration::from_secs(3))
        .idle_timeout(Some(Duration::from_secs(120)))
        .build(manager)
        .expect("Failed to create pool")
}

fn main() -> std::io::Result<()> {
    tokio::runtime::Runtime::new()?.block_on(async_main())
}

async fn async_main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let pool = establish_connection_pool();
    log::info!("Database connection pool established");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec!["Content-Type", "Authorization"])
                    .max_age(3600),
            )
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Compress::default())
            .wrap(RequestIdentifier::with_uuid())
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .route("/login", web::post().to(auth::login))
                            .route("/register", web::post().to(auth::register)),
                    )
                    .service(
                        web::scope("/v1").service(
                            web::resource("/students")
                                .route(web::get().to(student::get_students))
                                .route(web::post().to(student::create_student)),
                        ),
                    ),
            )
    })
    .bind("0.0.0.0:8081")?
    .workers(2)
    .run()
    .await
}
