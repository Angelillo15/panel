use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use crate::auth::error::AuthError;
use crate::auth::request::login::LoginRequest;
use crate::auth::request::register::RegisterRequest;
use crate::auth::request::renew::RenewRequest;
use crate::auth::service::auth::{AuthService, TokenResponse};
use crate::entity::user as User;

pub struct  SQLImpl {
    pub db: DatabaseConnection,
}

impl SQLImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AuthService for SQLImpl {
    async fn login(&self, login_request: &LoginRequest) -> Result<TokenResponse, AuthError> {
        let user: Option<User::Model> = User::Entity::find()
            .filter(User::Column::Username.eq(login_request.username.clone()).or(User::Column::Email.eq(login_request.username.clone())))
            .one(&self.db)
            .await.unwrap();

        match user {
            Some(user) => {
                Ok(TokenResponse {
                    long_token: Some("test".to_string()),
                    short_token: "test".to_string(),
                })
            }
            None => {
                Err(AuthError::InvalidUsernameOrPassword)
            }
        }
    }

    async fn register(&self, register_request: &RegisterRequest) -> Result<TokenResponse, AuthError> {
        todo!()
    }

    async fn renew(&self, renew_request: &RenewRequest) -> Result<TokenResponse, AuthError> {
        todo!()
    }
}