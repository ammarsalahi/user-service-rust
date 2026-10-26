mod common;
mod enums;
mod error;
mod models;
mod repositories;
mod routes;
mod traits;

use axum::Router;
use surrealdb::{Surreal, engine::any::connect, opt::auth::Root};

use routes::{auth_routes, user_routes, validation_routes};

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
        .with_state(db);

    // Start the HTTP server.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("Server running at http://localhost:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
