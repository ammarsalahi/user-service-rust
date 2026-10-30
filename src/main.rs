mod common;
mod enums;
mod error;
mod models;
mod repositories;
mod routes;
mod traits;

use axum::Router;
use surrealdb::{Surreal, engine::any::connect, opt::auth::Root};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use routes::{auth_routes, user_routes, validation_routes};

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::auth::login,
        routes::auth::register,
        routes::auth::forget_password,
        routes::auth::change_password,
        routes::auth::check_validation,
        routes::user::create_user,
        routes::user::update_user,
        routes::user::detail_user,
        routes::user::delete_user,
        routes::user::list_users,
        routes::validation::create_validation,
        routes::validation::update_validation,
        routes::validation::detail_validation,
        routes::validation::delete_validation,
        routes::validation::list_validations,
    ),
    components(schemas(
        models::auth::Login,
        models::auth::Register,
        models::auth::ChangePassword,
        models::auth::ForgetPassword,
        models::auth::AuthResponse,
        models::user::User,
        models::user::UpdateUser,
        models::user::UserResponse,
        models::validation::Validation,
        models::validation::CheckValidation,
        enums::Sex,
        enums::ActionType,
    )),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "validations", description = "Validation code endpoints"),
    ),
    info(
        title = "User Service API",
        version = "0.1.0",
        description = "A user management service built with Rust, Axum, and SurrealDB",
    ),
    servers(
        (url = "http://localhost:3000", description = "Local development server"),
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Environment configuration
    let surreal_url =
        std::env::var("SURREAL_URL").unwrap_or_else(|_| "ws://localhost:8000".to_string());

    let surreal_user = std::env::var("SURREAL_USER").unwrap_or_else(|_| "root".to_string());

    let surreal_password = std::env::var("SURREAL_PASSWORD").unwrap_or_else(|_| "root".to_string());

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    // Connect to SurrealDB
    let db: Surreal<surrealdb::engine::any::Any> = connect(&surreal_url).await?;

    // Authenticate
    db.signin(Root {
        username: &surreal_user,
        password: &surreal_password,
    })
    .await?;

    // Select namespace and database
    db.use_ns("app").use_db("main").await?;

    // Configure application routes
    let app: Router = Router::new()
        .merge(auth_routes())
        .merge(user_routes())
        .merge(validation_routes())
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(db);

    // Start HTTP server
    let address = format!("{host}:{port}");
    let listener = tokio::net::TcpListener::bind(&address).await?;

    println!("Server running at http://{address}");
    println!("Swagger UI available at http://{address}/swagger");

    axum::serve(listener, app).await?;

    Ok(())
}
