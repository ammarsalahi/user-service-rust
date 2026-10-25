use crate::enums::{Sex, ValidationType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Validation {
    pub email: Option<String>,
    pub username: Option<String>,
    pub r#type: ValidationType,
    pub is_expired: bool,
}
