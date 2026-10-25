use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Sex {
    Male,
    Female,
    Other,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ValidationType {
    Register(String),
    ForgetPassword(String),
}
