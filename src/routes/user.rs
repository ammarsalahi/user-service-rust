use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    common::DBState,
    error::Error,
    models::{UpdateUser, User, UserResponse},
    repositories::UserRepository,
    traits::DBActions,
};

// Create a new user.
#[utoipa::path(
    post,
    path = "/users",
    request_body = User,
    responses(
        (status = 200, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    tag = "users"
)]
pub async fn create_user(
    State(db): DBState,
    Json(data): Json<User>,
) -> Result<Json<UserResponse>, Error> {
    let repo = UserRepository::new(db);
    let user = repo.create(data).await?;

    Ok(Json(user))
}

// Update an existing user.
#[utoipa::path(
    put,
    path = "/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    request_body = UpdateUser,
    responses(
        (status = 200, description = "User updated successfully", body = Option<UserResponse>),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "users"
)]
pub async fn update_user(
    State(db): DBState,
    Path(id): Path<String>,
    Json(data): Json<UpdateUser>,
) -> Result<Json<Option<UserResponse>>, Error> {
    let repo = UserRepository::new(db);
    let user = repo.update(&id, data).await?;

    Ok(Json(user))
}

// Get a user by ID.
#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "users"
)]
pub async fn detail_user(
    State(db): DBState,
    Path(id): Path<String>,
) -> Result<Json<UserResponse>, Error> {
    let repo = UserRepository::new(db);

    let user = repo
        .read_one(&id)
        .await?
        .ok_or_else(|| Error::NotFound(format!("User with ID '{id}' not found")))?;

    Ok(Json(user))
}

// Delete a user by ID.
#[utoipa::path(
    delete,
    path = "/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "users"
)]
pub async fn delete_user(State(db): DBState, Path(id): Path<String>) -> Result<StatusCode, Error> {
    let repo = UserRepository::new(db);
    let is_deleted = repo.delete(&id).await?;

    if !is_deleted {
        return Err(Error::NotFound(format!("User with ID '{id}' not found")));
    }

    Ok(StatusCode::NO_CONTENT)
}

// List all users.
#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List of users", body = Vec<UserResponse>),
        (status = 500, description = "Internal server error")
    ),
    tag = "users"
)]
pub async fn list_users(State(db): DBState) -> Result<Json<Vec<UserResponse>>, Error> {
    let repo = UserRepository::new(db);
    let users = repo.read_all().await?;

    Ok(Json(users))
}
