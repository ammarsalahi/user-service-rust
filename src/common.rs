use axum::extract::State;
use surrealdb::{Surreal, engine::any::Any};

pub type DBState = State<Surreal<Any>>;
pub type DB = Surreal<Any>;
