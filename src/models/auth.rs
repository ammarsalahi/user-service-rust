use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Login {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Register {
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChangePassword {
    old_password: String,
    new_password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ForgetPassword {
    email: String,
}

pub struct AuthResponse {
    access: String,
    refresh: String,
}
