use actix_web::HttpRequest;
use base64::Engine;
use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::{
    database::user::User,
    error::{IntoHttpResult, ServerError},
};

#[derive(Default)]
struct Headers {
    pub signature: String, // SHA-256(login + nonce + SHA-256(password), base58), base58
    pub nonce: i64,
}

impl Headers {
    pub fn extract(request: &HttpRequest) -> actix_web::Result<Headers> {
        let mut headers = Headers::default();
        for (name, value) in request.headers() {
            match &*name.to_string() {
                "Signature" => {
                    headers.signature = value
                        .to_str()
                        .map_err(|_| ServerError::WrongSignature)
                        .into_http_400()?
                        .to_string()
                }
                "Nonce" => {
                    headers.nonce = value
                        .to_str()
                        .map_err(|_| ServerError::WrongNonce)
                        .into_http_400()?
                        .parse::<i64>()
                        .map_err(|_| ServerError::WrongNonce)
                        .into_http_400()?
                }
                _ => {}
            }
        }
        Ok(headers)
    }
}

pub async fn authorize(
    login: &str,
    request: &HttpRequest,
    pool: &PgPool,
) -> actix_web::Result<User> {
    let user = User::get(login, pool)
        .await
        .into_http_500()?
        .ok_or_else(|| ServerError::UserNotFound(login.to_string()))
        .into_http_400()?;

    let headers = Headers::extract(request)?;

    let mut correct_signature = Sha256::new();
    correct_signature.update(&format!("{}{}{}", login, headers.nonce, user.data.password));
    let correct_signature = correct_signature.finalize().to_vec();
    let correct_signature = base64::engine::general_purpose::STANDARD.encode(correct_signature);

    if correct_signature != headers.signature {
        return ServerError::WrongSignature.into_http_400();
    }

    let nonce_valid = user
        .update_auth_nonce(headers.nonce, pool)
        .await
        .into_http_500()?;

    if !nonce_valid {
        return ServerError::WrongNonce.into_http_400();
    }

    Ok(user)
}
