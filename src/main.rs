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
    // Connect to SurrealDB.
    let db: Surreal<surrealdb::engine::any::Any> = connect("ws://127.0.0.1:8000").await?;

    // Authenticate with SurrealDB.
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select the namespace and database.
    db.use_ns("app").use_db("main").await?;

    // Configure the application routes.
    let app: Router = Router::new()
        .merge(auth_routes())
        .merge(user_routes())
        .merge(validation_routes())
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(db);

    // Start the HTTP server.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("Server running at http://localhost:3000");
    println!("Swagger UI available at http://localhost:3000/swagger");

    axum::serve(listener, app).await?;

    Ok(())
}
