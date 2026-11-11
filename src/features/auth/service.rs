use crate::config::AppConfig;
use crate::features::auth::dto::{Claims, LoginRequest};
use crate::features::auth::jwt::create_token;
use crate::features::users::dto::CreateUserRequest;
use crate::features::users::entity::User;
use crate::features::users::service::UserService;
use crate::shared::error::AppError;
use crate::shared::security::verify_password;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct AuthService {
    config: Arc<AppConfig>,
    user_service: Arc<UserService>,
}

impl AuthService {
    pub fn new(config: Arc<AppConfig>, user_service: Arc<UserService>) -> Self {
        Self {
            config,
            user_service,
        }
    }

    pub async fn register(&self, user_payload: CreateUserRequest) -> Result<String, AppError> {
        let user = self.user_service.create_user(user_payload).await?;

        // TODO Make struct for the token, and send to FE the exp time
        self.generate_token(&user)
    }

    pub async fn login(&self, credentials: LoginRequest) -> Result<String, AppError> {
        let found_user = self
            .user_service
            .get_user_by_email(&credentials.email)
            .await
            .map_err(|error| match error {
                AppError::NotFound(_) => AppError::UnAuthorized("Incorrect email or password".to_string()),
                other => other
            })?;

        // TODO: timing attack: for an unknown email we return early (fast), while a known
        //  email runs bcrypt (slow), so response time reveals which emails exist.
        //  Fix: run a dummy bcrypt verify when the user is not found.
        let is_authenticated =
            verify_password(&credentials.password, &found_user.password_hash).await?;

        if !is_authenticated {
            return Err(AppError::UnAuthorized(
                "Incorrect email or password".to_string(),
            ));
        }

        // TODO Make struct for the token, and send to FE the exp time
        self.generate_token(&found_user)
    }

    fn generate_token(&self, user: &User) -> Result<String, AppError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let claims = Claims {
            sub: user.id.to_string(),
            exp: now + self.config.jwt_lifetime,
        };

        create_token(&claims, self.config.jwt_secret.as_bytes())
            .map_err(|e| AppError::InternalServer(e.to_string()))
    }
}
