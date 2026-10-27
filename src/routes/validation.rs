use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    common::DBState, error::Error, models::Validation, repositories::ValidationRepository,
    traits::DBActions,
};

// Create a new validation record (a code is auto-generated in the repo).
#[utoipa::path(
    post,
    path = "/validations",
    request_body = Validation,
    responses(
        (status = 200, description = "Validation created successfully", body = Validation),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    tag = "validations"
)]
pub async fn create_validation(
    State(db): DBState,
    Json(data): Json<Validation>,
) -> Result<Json<Validation>, Error> {
    let repo = ValidationRepository::new(db);
    let validation = repo.create(data).await?;

    Ok(Json(validation))
}

// Regenerate and replace a validation record by ID.
#[utoipa::path(
    put,
    path = "/validations/{id}",
    params(
        ("id" = String, Path, description = "Validation ID")
    ),
    request_body = Validation,
    responses(
        (status = 200, description = "Validation updated successfully", body = Validation),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "Validation not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "validations"
)]
pub async fn update_validation(
    State(db): DBState,
    Path(id): Path<String>,
    Json(data): Json<Validation>,
) -> Result<Json<Validation>, Error> {
    let repo = ValidationRepository::new(db);

    let validation = repo
        .update(&id, data)
        .await?
        .ok_or_else(|| Error::NotFound(format!("Validation with ID '{id}' not found")))?;

    Ok(Json(validation))
}

// Fetch a single validation record by ID.
#[utoipa::path(
    get,
    path = "/validations/{id}",
    params(
        ("id" = String, Path, description = "Validation ID")
    ),
    responses(
        (status = 200, description = "Validation found", body = Validation),
        (status = 404, description = "Validation not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "validations"
)]
pub async fn detail_validation(
    State(db): DBState,
    Path(id): Path<String>,
) -> Result<Json<Validation>, Error> {
    let repo = ValidationRepository::new(db);

    let validation = repo
        .read_one(&id)
        .await?
        .ok_or_else(|| Error::NotFound(format!("Validation with ID '{id}' not found")))?;

    Ok(Json(validation))
}

// Delete a validation record by ID.
#[utoipa::path(
    delete,
    path = "/validations/{id}",
    params(
        ("id" = String, Path, description = "Validation ID")
    ),
    responses(
        (status = 204, description = "Validation deleted successfully"),
        (status = 404, description = "Validation not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "validations"
)]
pub async fn delete_validation(
    State(db): DBState,
    Path(id): Path<String>,
) -> Result<StatusCode, Error> {
    let repo = ValidationRepository::new(db);

    let is_deleted = repo.delete(&id).await?;

    if !is_deleted {
        return Err(Error::NotFound(format!(
            "Validation with ID '{id}' not found"
        )));
    }

    Ok(StatusCode::NO_CONTENT)
}

// List every validation record.
#[utoipa::path(
    get,
    path = "/validations",
    responses(
        (status = 200, description = "List of validations", body = Vec<Validation>),
        (status = 500, description = "Internal server error")
    ),
    tag = "validations"
)]
pub async fn list_validations(State(db): DBState) -> Result<Json<Vec<Validation>>, Error> {
    let repo = ValidationRepository::new(db);

    let validations = repo.read_all().await?;

    Ok(Json(validations))
}
