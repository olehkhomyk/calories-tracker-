use crate::config::AppConfig;
use crate::features::auth::dto::Claims;
use crate::features::auth::jwt::create_token;
use crate::features::users::dto::CreateUserDto;
use crate::features::users::service::UserService;
use crate::shared::error::AppError;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct AuthService {
    config: Arc<AppConfig>,
    user_service: Arc<UserService>,
}

impl AuthService {
    pub fn new(config: Arc<AppConfig>, user_service: Arc<UserService>) -> Self {
        Self { config, user_service }
    }

    pub async fn register(&self, user: CreateUserDto) -> Result<String, AppError> {
        let user = self.user_service.create_user(user).await?;

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        let claims = Claims {
            sub: user.id.to_string(),
            exp: now + 60 * 60 * 24, // 24h
        };

        let token = create_token(&claims, self.config.jwt_secret.as_bytes())
            .map_err(|e| AppError::InternalServer(e.to_string()))?;

        Ok(token)
    }
}

