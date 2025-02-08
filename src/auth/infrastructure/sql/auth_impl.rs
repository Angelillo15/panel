use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use crate::auth::error::AuthError;
use crate::auth::service::auth::{AuthService, LoginRequest, RegisterRequest, RenewRequest, TokenResponse};
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
    async fn login(&self, login_request: LoginRequest) -> Result<TokenResponse, AuthError> {
        let user: Option<User::Model> = User::Entity::find()
            .filter(User::Column::Username.eq(login_request.username.clone()).or(User::Column::Email.eq(login_request.username.clone())))
            .one(&self.db)
            .await.unwrap();


        Ok(TokenResponse {
            long_token: None,
            short_token: "".to_string(),
        })
    }

    async fn register(&self, register_request: RegisterRequest) -> Result<TokenResponse, AuthError> {
        todo!()
    }

    async fn renew(&self, renew_request: RenewRequest) -> Result<TokenResponse, AuthError> {
        todo!()
    }
}