use axum::{Json, extract::State, http::StatusCode};

use crate::{common::DBState, error::Error, models::*, repositories::AuthRepository};

/// POST /auth/login
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
