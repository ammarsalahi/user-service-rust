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
pub async fn create_user(
    State(db): DBState,
    Json(data): Json<User>,
) -> Result<Json<UserResponse>, Error> {
    let repo = UserRepository::new(db);
    let user = repo.create(data).await?;

    Ok(Json(user))
}

// Update an existing user.
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
pub async fn delete_user(State(db): DBState, Path(id): Path<String>) -> Result<StatusCode, Error> {
    let repo = UserRepository::new(db);
    let is_deleted = repo.delete(&id).await?;

    if !is_deleted {
        return Err(Error::NotFound(format!("User with ID '{id}' not found")));
    }

    Ok(StatusCode::NO_CONTENT)
}

// List all users.
pub async fn list_users(State(db): DBState) -> Result<Json<Vec<UserResponse>>, Error> {
    let repo = UserRepository::new(db);
    let users = repo.read_all().await?;

    Ok(Json(users))
}
