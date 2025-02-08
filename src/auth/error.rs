use thiserror::Error;

#[derive(Debug, Error)]
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