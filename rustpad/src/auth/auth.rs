use warp::{Filter, Rejection, Reply};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject (typically the user ID or email)
    exp: usize,  // Expiration time (in seconds since epoch)
}

/// Secret key for signing tokens, loaded from an environment variable for security
fn get_secret_key() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| "your_secret_key".to_string())  // Default key, replace with a secure one
}

/// Generates a JWT token for the given user ID
pub fn generate_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))  // Token valid for 24 hours
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(get_secret_key().as_ref()))
}

/// Validates the given JWT token and returns the claims if valid
pub fn validate_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_secret_key().as_ref()),
        &Validation::default(),
    )
}

/// Filter for requiring JWT authentication in routes
pub fn with_auth() -> impl Filter<Extract = (Claims,), Error = Rejection> + Clone {
    warp::header::<String>("authorization")
        .and_then(|token: String| async move {
            match validate_jwt(&token) {
                Ok(token_data) => Ok(token_data.claims),
                Err(_) => Err(warp::reject::custom(AuthError::InvalidToken)),
            }
        })
}

/// Custom error type for handling auth errors
#[derive(Debug)]
struct AuthError {
    message: String,
}

impl warp::reject::Reject for AuthError {}

impl AuthError {
    fn invalid_token() -> Self {
        AuthError {
            message: "Invalid token".to_string(),
        }
    }
}

pub async fn login_handler(user_id: String) -> Result<impl Reply, Rejection> {
    match generate_jwt(&user_id) {
        Ok(token) => Ok(warp::reply::json(&token)),
        Err(_) => Err(warp::reject::custom(AuthError::invalid_token())),
    }
}

/// This will be used to protect routes that require authentication
pub fn protected_route() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    warp::path("protected")
        .and(with_auth())  // Require JWT authentication
        .map(|claims: Claims| format!("Welcome, user {}!", claims.sub))
}
