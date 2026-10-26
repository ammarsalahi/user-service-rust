use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use crate::{
    common::DB,
    enums::Sex,
    error::Error,
    models::{Register, User},
};

pub struct AuthRepository {
    db: DB,
}

impl AuthRepository {
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    /// Hash a plaintext password using Argon2.
    fn hash_password(password: &str) -> Result<String, Error> {
        let salt = SaltString::generate(&mut OsRng);

        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|_| Error::InternalServerError("Failed to hash password".to_string()))
    }

    /// Verify a plaintext password against its Argon2 hash.
    fn verify_password(password: &str, password_hash: &str) -> Result<bool, Error> {
        let parsed_hash = PasswordHash::new(password_hash)
            .map_err(|_| Error::InternalServerError("Invalid stored password hash".to_string()))?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Register a new user with an unconfirmed account.
    pub async fn register(&self, data: Register) -> Result<User, Error> {
        // Check whether the email is already registered.
        let existing_users: Vec<User> = self
            .db
            .query("SELECT * FROM user WHERE email = $email LIMIT 1")
            .bind(("email", data.email.clone()))
            .await?
            .take(0)?;

        if !existing_users.is_empty() {
            return Err(Error::InvalidInput(
                "Email is already registered".to_string(),
            ));
        }

        // Never store plaintext passwords.
        let password_hash = Self::hash_password(&data.password)?;

        let user = User {
            name: Some(data.name),
            username: data.email.clone(),
            password: password_hash,
            sex: Sex::Other,
            email: data.email,
            is_admin: Some(false),
            is_confirmed: Some(false),
        };

        let created_user: Option<User> = self.db.create("user").content(user).await?;

        created_user.ok_or_else(|| Error::InternalServerError("Failed to create user".to_string()))
    }

    /// Authenticate a confirmed user.
    ///
    /// Returns None when the email does not exist, the account
    /// is unconfirmed, or the password is incorrect.
    pub async fn login(&self, email: &str, password: &str) -> Result<Option<User>, Error> {
        let mut users: Vec<User> = self
            .db
            .query(
                "SELECT * FROM user \
                 WHERE email = $email \
                 AND is_confirmed = true \
                 LIMIT 1",
            )
            .bind(("email", email.to_string()))
            .await?
            .take(0)?;

        let Some(user) = users.pop() else {
            return Ok(None);
        };

        if Self::verify_password(password, &user.password)? {
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    /// Find a user by email for a password-reset workflow.
    ///
    /// The calling handler must not disclose whether the account
    /// exists in its public response.
    pub async fn request_reset(&self, email: &str) -> Result<Option<User>, Error> {
        let mut users: Vec<User> = self
            .db
            .query("SELECT * FROM user WHERE email = $email LIMIT 1")
            .bind(("email", email.to_string()))
            .await?
            .take(0)?;

        Ok(users.pop())
    }

    /// Change a user's password.
    ///
    /// `new_password` is plaintext and is hashed inside this method.
    /// The caller must authorize the request before invoking it.
    pub async fn change_password(&self, user_id: &str, new_password: &str) -> Result<bool, Error> {
        let password_hash = Self::hash_password(new_password)?;

        let updated: Option<User> = self
            .db
            .update(("user", user_id))
            .merge(serde_json::json!({
                "password": password_hash
            }))
            .await?;

        Ok(updated.is_some())
    }
}
