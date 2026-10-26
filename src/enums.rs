use serde::{Deserialize, Serialize};

/// User's gender, stored directly on the user record.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Sex {
    Male,
    Female,
    Other,
}

/// Kind of verification flow; the String carries the target email.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ActionType {
    Register(String),
    ForgetPassword(String),
}
