use axum::{Json, extract::State, http::StatusCode};
use utoipa::OpenApi;

use crate::{common::DBState, error::Error, models::*, repositories::AuthRepository};

/// POST /auth/login
#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = Login,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 400, description = "Invalid input"),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server error")
    ),
    tag = "auth"
)]
pub async fn login(
    State(db): DBState,
    Json(data): Json<Login>,
) -> Result<Json<AuthResponse>, Error> {
    if data.email.trim().is_empty() || data.password.is_empty() {
        return Err(Error::InvalidInput(
            "Email and password are required".into(),
        ));
    }

    let repository = AuthRepository::new(db);

    let user = repository
        .login(data.email.trim(), &data.password)
        .await?
        .ok_or(Error::InvalidCredentials)?;

    // AuthResponse must exclude the user's password hash.
    let response = AuthResponse::from(user).map_err(|_| {
        Error::InternalServerError("Failed to generate authentication tokens".into())
    })?;

    Ok(Json(response))
}

/// POST /auth/register
#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = Register,
    responses(
        (status = 200, description = "Registration successful", body = AuthResponse),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    tag = "auth"
)]
pub async fn register(
    State(db): DBState,
    Json(data): Json<Register>,
) -> Result<Json<AuthResponse>, Error> {
    if data.name.trim().is_empty() || data.email.trim().is_empty() || data.password.is_empty() {
        return Err(Error::InvalidInput(
            "Name, email, and password are required".into(),
        ));
    }

    if data.password.len() < 8 {
        return Err(Error::InvalidInput(
            "Password must contain at least 8 bytes".into(),
        ));
    }

    let repository = AuthRepository::new(db);

    let user = repository.register(data).await?;

    let response = AuthResponse::from(user).map_err(|_| {
        Error::InternalServerError("Failed to generate authentication tokens".into())
    })?;

    Ok(Json(response))
}

/// POST /auth/forget-password
#[utoipa::path(
    post,
    path = "/auth/forget-password",
    request_body = ForgetPassword,
    responses(
        (status = 202, description = "Password reset requested"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    tag = "auth"
)]
pub async fn forget_password(
    State(db): DBState,
    Json(data): Json<ForgetPassword>,
) -> Result<StatusCode, Error> {
    if data.email.trim().is_empty() {
        return Err(Error::InvalidInput("Email is required".into()));
    }

    let repository = AuthRepository::new(db);

    // This only looks up the user. A complete password-reset
    // workflow must also issue and deliver a secure reset token.
    let _user = repository.request_reset(data.email.trim()).await?;

    // Do not disclose whether an account exists.
    Ok(StatusCode::ACCEPTED)
}

/// POST /auth/change-password
#[utoipa::path(
    post,
    path = "/auth/change-password",
    request_body = ChangePassword,
    responses(
        (status = 204, description = "Password changed successfully"),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "auth"
)]
pub async fn change_password(
    State(db): DBState,
    Json(data): Json<ChangePassword>,
) -> Result<StatusCode, Error> {
    if data.new_password.is_empty() {
        return Err(Error::InvalidInput("New password is required".into()));
    }

    if data.new_password.len() < 8 {
        return Err(Error::InvalidInput(
            "Password must contain at least 8 bytes".into(),
        ));
    }

    let repository = AuthRepository::new(db);

    let changed = repository
        .change_password(&data.user_id, &data.new_password)
        .await?;

    if !changed {
        return Err(Error::NotFound("User not found".into()));
    }

    Ok(StatusCode::NO_CONTENT)
}

/// POST /auth/validation/check
///
/// Request:
/// {
///     "code": "123456"
/// }
///
/// Returns 200 if the code exists, or 404 if it does not.
#[utoipa::path(
    post,
    path = "/auth/validation/check",
    request_body = CheckValidation,
    responses(
        (status = 200, description = "Validation code is valid"),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "Invalid validation code"),
        (status = 500, description = "Internal server error")
    ),
    tag = "auth"
)]
pub async fn check_validation(
    State(db): DBState,
    Json(data): Json<CheckValidation>,
) -> Result<StatusCode, Error> {
    let code = data.code.trim();

    if code.is_empty() {
        return Err(Error::InvalidInput("Validation code is required".into()));
    }

    let mut response = db
        .query(
            "SELECT * FROM validation \
             WHERE code = $code \
             LIMIT 1",
        )
        .bind(("code", code.to_string()))
        .await?;

    let validation: Option<Validation> = response.take(0)?;

    match validation {
        Some(_) => Ok(StatusCode::OK),
        None => Err(Error::NotFound("Invalid validation code".into())),
    }
}
