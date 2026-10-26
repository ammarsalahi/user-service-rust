use crate::enums::Sex;
use serde::{Deserialize, Serialize};

// Full user record as stored in the database.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub name: Option<String>,
    pub username: String,
    pub password: String,
    pub sex: Sex,
    pub email: String,
    pub is_admin: Option<bool>,
    pub is_confirmed: Option<bool>,
}

// Fields allowed to change on an existing user.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub username: String,
    pub sex: Sex,
    pub email: String,
    pub is_admin: Option<bool>,
    pub is_confirmed: Option<bool>,
}

// Public user shape; deliberately excludes the password hash.
#[derive(Debug, Serialize, Clone)]
pub struct UserResponse {
    pub name: Option<String>,
    pub username: String,
    pub sex: Sex,
    pub email: String,
    pub is_admin: Option<bool>,
    pub is_confirmed: Option<bool>,
}

// Strip sensitive fields when converting to the public shape.
impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            name: user.name,
            username: user.username,
            sex: user.sex,
            email: user.email,
            is_admin: user.is_admin,
            is_confirmed: user.is_confirmed,
        }
    }
}
