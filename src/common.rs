use axum::extract::State;
use surrealdb::{Surreal, engine::any::Any};

// Shared type aliases for the SurrealDB connection.

// Axum extractor bound to the app's database state.
pub type DBState = State<Surreal<Any>>;
// Handle used by repositories to talk to SurrealDB.
pub type DB = Surreal<Any>;
