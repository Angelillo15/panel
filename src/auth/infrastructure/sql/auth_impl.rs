use crate::auth::error::AuthError;
use crate::auth::request::login::LoginRequest;
use crate::auth::request::register::RegisterRequest;
use crate::auth::request::renew::RenewRequest;
use crate::auth::service::auth::{AuthService, TokenResponse};
use crate::auth::service::hash_service::HashService;
use crate::entity::session as Session;
use crate::entity::user as User;
use async_trait::async_trait;
use chrono::Utc;
use log::error;
use sea_orm::sqlx::types::chrono::DateTime;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::sync::Arc;
use uuid::Uuid;

pub struct SQLImpl {
    db: DatabaseConnection,
    hash_service: Arc<HashService>,
}

impl SQLImpl {
    pub fn new(db: DatabaseConnection, hash_service: Arc<HashService>) -> Self {
        Self { db, hash_service }
    }
}

#[async_trait]
impl AuthService for SQLImpl {
    async fn login(&self, login_request: &LoginRequest) -> Result<TokenResponse, AuthError> {
        let user: Option<User::Model> = self
            .get_user_by_username_or_email(&*login_request.username, &*login_request.username)
            .await;

        match user {
            Some(user) => Ok(TokenResponse {
                long_token: Some("test".to_string()),
                short_token: "test".to_string(),
            }),
            None => Err(AuthError::InvalidUsernameOrPassword),
        }
    }

    async fn register(
        &self,
        register_request: &RegisterRequest,
    ) -> Result<TokenResponse, AuthError> {
        let user: Option<User::Model> = self
            .get_user_by_username_or_email(&*register_request.username, &*register_request.username)
            .await;

        if let Some(user) = user {
            if (*register_request.username).eq(&user.username) {
                return Err(AuthError::UsernameAlreadyInUse);
            }

            if (*register_request.email).eq(&register_request.email) {
                return Err(AuthError::EmailAlreadyInUse);
            }
        }

        let hashed_password = self.hash_service.hash_password(&*register_request.password);

        if let Err(e) = hashed_password {
            error!("Error hashing password: {}", e);
            return Err(AuthError::InternalError);
        }

        let new_user = User::ActiveModel {
            email: Set(register_request.email.to_string()),
            uuid: Set(Uuid::new_v4()),
            username: Set(register_request.username.to_string()),
            password: Set(hashed_password.unwrap()),
            created_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        if let Err(err) = new_user.save(&self.db).await {
            error!("Error saving user: {}", err);
            return Err(AuthError::InternalError);
        }

        Ok(TokenResponse {
            long_token: Some("a".to_string()),
            short_token: "b".to_string(),
        })
    }

    async fn renew(&self, renew_request: &RenewRequest) -> Result<TokenResponse, AuthError> {
        todo!()
    }
}

impl SQLImpl {
    async fn get_user_by_username_or_email(
        &self,
        username: &str,
        email: &str,
    ) -> Option<User::Model> {
        let user: Option<User::Model> = User::Entity::find()
            .filter(
                User::Column::Username
                    .eq(username)
                    .or(User::Column::Email.eq(email)),
            )
            .one(&self.db)
            .await
            .unwrap();

        user
    }

    async fn create_session_for_user(
        &self,
        user: &User::Model,
        long_token: bool,
    ) -> Result<TokenResponse, AuthError> {
        todo!("Needs to be implemented")
    }
}
