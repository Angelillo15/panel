use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use strum::{AsRefStr, EnumString};
use thiserror::Error;

#[derive(Debug, Error, AsRefStr, EnumString)]
pub enum AuthError {
  #[error("Invalid username or password")]
  InvalidUsernameOrPassword,
  #[error("Invalid email")]
  InvalidEmail,
  #[error("Invalid password, check the password requirements")]
  InvalidPassword,
  #[error("Invalid captcha")]
  InvalidCaptcha,
  #[error("Invalid token")]
  InvalidToken,
  #[error("Token expired")]
  TokenExpired,
  #[error("Token not found")]
  TokenNotFound,
  #[error("Token not valid")]
  TokenNotValid,
  #[error("No private key was provided")]
  NoPrivateKey,
}

impl AuthError {
  pub fn i18n_key(&self) -> String {
    format!("auth.{}", self.as_ref().to_lowercase())
  }
}

impl Serialize for AuthError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
      S: Serializer
  {
    let error_message = self.to_string();
    let i18n_key = self.i18n_key();

    let mut map = serializer.serialize_map(Some(2))?;
    map.serialize_entry("error", &error_message)?;
    map.serialize_entry("key", &i18n_key)?;
    map.end()
  }
}