use crate::config::AppConfig;
use crate::features::auth::dto::{Claims, LoginDto};
use crate::features::auth::jwt::create_token;
use crate::features::users::dto::CreateUserDto;
use crate::features::users::service::UserService;
use crate::shared::error::AppError;
use crate::shared::security::hash_password;
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

    pub async fn register(&self, user_payload: CreateUserDto) -> Result<String, AppError> {
        let user = self.user_service.create_user(user_payload).await?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let claims = Claims {
            sub: user.id.to_string(),
            exp: now + self.config.jwt_lifetime,
        };

        let token = create_token(&claims, self.config.jwt_secret.as_bytes())
            .map_err(|e| AppError::InternalServer(e.to_string()))?;

        // TODO Make struct for the token, and send to FE the exp time
        Ok(token)
    }

    pub async fn login(&self, credentials: LoginDto) -> Result<(), AppError> {
        // TODO implement check and create signed token
        let found_user = self.user_service.get_by_user_by_email(&credentials.email).await?;
        let hashed_pass = hash_password(credentials.password).await?;
        // TODO: User is not returning the password, so think about it and implement the way
        //  to return the password so it will be possible to compare them.
        // let is_authenticated = verify_password(&credentials.password, found_user.password)



        Ok(())
    }
}
