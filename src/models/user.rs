use crate::enums::Sex;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub username: String,
    pub sex: Sex,
    pub email: String,
    pub is_admin: Option<bool>,
    pub is_confirmed: Option<bool>,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserResponse {
    pub name: Option<String>,
    pub username: String,
    pub sex: Sex,
    pub email: String,
    pub is_admin: Option<bool>,
    pub is_confirmed: Option<bool>,
}

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
