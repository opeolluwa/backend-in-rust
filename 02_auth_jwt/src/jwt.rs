use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{config::CONFIG, error::AppError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub user_email: String,
    pub user_id: String,
    exp: usize
}

impl JwtClaims {
    pub fn new(user_email: String, user_id: String) -> Self {
        Self {
            user_email,
            user_id,
             exp: 2000000000, // May 2
        }
    }

    /// generate a  new token
    pub fn gen_token(&self) -> Result<std::string::String, jsonwebtoken::errors::Error> {
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(CONFIG.jwt_signing_key.as_bytes()),
        )
    }

    /// get the claim from the token
    pub fn parse_token(token: String) -> Result<JwtClaims, AppError> {
        match decode::<Self>(
            &token,
            &DecodingKey::from_secret(CONFIG.jwt_signing_key.as_bytes()),
            &Validation::default(),
        ) {
            Ok(claim) => Ok(claim.claims),
            Err(_) => Err(AppError::ServerError { message: None }),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for JwtClaims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header

        let Ok(TypedHeader(Authorization(bearer))) =
            parts.extract::<TypedHeader<Authorization<Bearer>>>().await
        else {
            return Err(AppError::WrongCredentialsError {
                message: Some("Invalid or expired authorization header".to_string()),
            });
        };

        let user_data = JwtClaims::parse_token(bearer.token().to_string()).map_err(|_| {
            AppError::WrongCredentialsError {
                message: Some("You are not authorized to access this resource! Please login and retry".to_string()),
            }
        });

        user_data
    }
}
