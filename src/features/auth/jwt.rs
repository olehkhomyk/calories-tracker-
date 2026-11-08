use crate::features::auth::model::Claims;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};

pub fn create_token(
    claims: &Claims,
    secret: &[u8],
) -> Result<String, jsonwebtoken::errors::Error> {
    encode(
        &Header::new(Algorithm::HS256),
        claims,
        &EncodingKey::from_secret(secret),
    )
}

pub fn verify_token(
    token: &str,
    secret: &[u8],
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &validation,
    )?;

    Ok(data.claims)
}
