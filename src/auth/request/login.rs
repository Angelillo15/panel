use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct LoginRequest {
    #[validate(length(min = 4, max = 150))]
    pub username: String,
    #[validate(length(min = 4, max = 30))]
    pub password: String,
    pub remember_me: bool,
}
