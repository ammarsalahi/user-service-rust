use crate::models::User;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// JWT payload; token_type distinguishes access from refresh tokens.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub token_type: String,
}

// Request bodies for the auth endpoints.
#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct Register {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct ChangePassword {
    pub user_id: String,
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct ForgetPassword {
    pub email: String,
}

// Successful auth response carrying both JWTs.
#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct AuthResponse {
    pub access: String,
    pub refresh: String,
}

impl AuthResponse {
    pub fn from(user: User) -> Result<Self, jsonwebtoken::errors::Error> {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let now = Utc::now();

        // Create short-lived access-token claims.
        let access_claims = Claims {
            sub: user.username.to_string(),
            exp: (now + Duration::hours(1)).timestamp() as usize,
            token_type: "access".to_string(),
        };

        // Create long-lived refresh-token claims.
        let refresh_claims = Claims {
            sub: user.username.to_string(),
            exp: (now + Duration::hours(24)).timestamp() as usize,
            token_type: "refresh".to_string(),
        };

        let key = EncodingKey::from_secret(secret.as_bytes());

        let access = encode(&Header::default(), &access_claims, &key)?;
        let refresh = encode(&Header::default(), &refresh_claims, &key)?;

        Ok(Self { access, refresh })
    }
}
