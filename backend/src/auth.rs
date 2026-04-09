use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::Json;
use axum::extract::FromRequestParts;
use axum::extract::State;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;
use tracing::{error, info, instrument, warn};
use validator::Validate;

use crate::state::AppState;

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    #[validate(email(message = "couldnt validate email"))]
    pub email: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt_cookie(username: &str) -> Result<Cookie<'static>, (StatusCode, String)> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: username.to_string(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| {
        error!("Token creation failed for user '{}': {}", username, e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Token creation error: {}", e),
        )
    })?;

    let cookie = Cookie::build(("jwt", token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::hours(24))
        .build();

    Ok(cookie)
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Cookie extraction failed".into(),
                )
            })?;

        let cookie = jar.get("jwt").ok_or((
            StatusCode::UNAUTHORIZED,
            "Unauthorized: Missing JWT cookie".into(),
        ))?;

        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");

        let token_data = decode::<Claims>(
            cookie.value(),
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| {
            warn!("Failed to decode token: {}", e);
            (
                StatusCode::UNAUTHORIZED,
                "Unauthorized: Invalid token".into(),
            )
        })?;

        Ok(token_data.claims)
    }
}

#[instrument(skip(state, jar, request))]
pub async fn register(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<RegisterRequest>,
) -> Result<(CookieJar, Json<serde_json::Value>), (StatusCode, String)> {
    if let Err(errors) = request.validate() {
        warn!("Validation failed for registration: {}", errors);
        return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(request.password.as_bytes(), &salt)
        .map_err(|err| {
            error!("Password hashing error: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        })?
        .to_string();

    let result = sqlx::query!(
        "INSERT INTO users (username, email, password_hash) VALUES (?,?,?)",
        request.username,
        request.email,
        password_hash
    )
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => {
            info!("User '{}' successfully registered", request.username);
            let cookie = create_jwt_cookie(&request.username)?;
            Ok((
                jar.add(cookie),
                Json(
                    serde_json::json!({ "message": "User registered and logged in successfully" }),
                ),
            ))
        }
        Err(e) => {
            warn!("Failed to register user '{}': {}", request.username, e);
            Err((StatusCode::CONFLICT, "Such user already exists".into()))
        }
    }
}

#[instrument(skip(state, jar, request))]
pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Result<(CookieJar, Json<serde_json::Value>), (StatusCode, String)> {
    let record = sqlx::query!(
        "SELECT password_hash FROM users WHERE username = ?",
        request.username
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        error!("DB error during login for '{}': {}", request.username, e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    let user = match record {
        Some(u) => u,
        None => {
            warn!("Login failed: user '{}' not found", request.username);
            return Err((
                StatusCode::UNAUTHORIZED,
                "Invalid username or password".into(),
            ));
        }
    };

    let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|e| {
        error!("Failed to parse hash for '{}': {}", request.username, e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Invalid hash format in DB".into(),
        )
    })?;

    if Argon2::default()
        .verify_password(request.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        warn!("Login failed: wrong password for '{}'", request.username);
        return Err((
            StatusCode::UNAUTHORIZED,
            "Invalid username or password".into(),
        ));
    }

    info!("User '{}' successfully logged in", request.username);
    let cookie = create_jwt_cookie(&request.username)?;
    Ok((
        jar.add(cookie),
        Json(serde_json::json!({ "message": "Login successful" })),
    ))
}
