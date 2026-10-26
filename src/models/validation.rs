use crate::enums::ActionType;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// One-time verification record for a pending auth action.
#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct Validation {
    pub email: Option<String>,
    pub user_id: Option<String>,
    pub action: ActionType,
    pub is_expired: Option<bool>,
    pub code: String,
}

// Body for submitting a code back for verification.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CheckValidation {
    pub code: String,
}
