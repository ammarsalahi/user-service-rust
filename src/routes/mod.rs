use axum::{
    Router,
    routing::{get, post},
};

use crate::common::DB;

mod auth;
mod user;
mod validation;

// Auth endpoints: login, register, and password flows.
pub fn auth_routes() -> Router<DB> {
    Router::new()
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
        .route("/auth/forget-password", post(auth::forget_password))
        .route("/auth/change-password", post(auth::change_password))
        .route("/auth/validation/check", post(auth::check_validation))
}

pub fn user_routes() -> Router<DB> {
    Router::new()
        .route("/users", post(user::create_user).get(user::list_users))
        .route(
            "/users/{id}",
            get(user::delete_user)
                .put(user::update_user)
                .delete(user::delete_user),
        )
}

pub fn validation_routes() -> Router<DB> {
    Router::new()
        .route(
            "/validations",
            post(validation::create_validation).get(validation::list_validations),
        )
        .route(
            "/validations/{id}",
            get(validation::detail_validation)
                .put(validation::update_validation)
                .delete(validation::delete_validation),
        )
}
