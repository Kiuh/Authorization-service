use actix_web::{web, HttpRequest, HttpResponse};
use itertools::Itertools;
use sha2::{Digest, Sha256};

use crate::{
    database::user::User,
    error::{ResponseError, ServerError},
    server_state::ServerState,
};

#[derive(Default)]
struct Headers {
    pub signature: String,
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
                        .map_err(|_| ServerError::WrongSignature.http_status_400())?
                        .to_string()
                }
                "Nonce" => {
                    headers.nonce = value
                        .to_str()
                        .map_err(|_| ServerError::WrongNonce.http_status_400())?
                        .parse::<i64>()
                        .map_err(|_| ServerError::WrongNonce.http_status_400())?
                }
                _ => {}
            }
        }
        Ok(headers)
    }
}

// Headers:
// Signature: SHA-256(login + nonce + SHA-256(password), base58), base58
// Nonce: i64
pub async fn execute(
    st: web::Data<ServerState>,
    request: HttpRequest,
    request_body: actix_web::web::Bytes,
) -> actix_web::Result<HttpResponse> {
    let mut path_iter = request.path().split('/');
    let (_, user_literal, login) = (path_iter.next(), path_iter.next(), path_iter.next());
    if Some("User") != user_literal || login.is_none() {
        return Err(ServerError::WrongRequest.http_status_404().into());
    }
    let login = login.unwrap();

    let user = User::get(&login, &st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?
        .ok_or_else(|| ServerError::UserNotFound(login.to_string()).http_status_400())?;

    let headers = Headers::extract(&request)?;

    let mut correct_signature = Sha256::new();
    correct_signature.update(&format!("{}{}{}", login, headers.nonce, user.data.password));
    let correct_signature = correct_signature.finalize().to_vec();
    let correct_signature = bs58::encode(correct_signature).into_string();

    if correct_signature != headers.signature {
        return Err(ServerError::WrongSignature.http_status_400().into());
    }

    let nonce_valid = user
        .update_auth_nonce(headers.nonce, &st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?;

    if !nonce_valid {
        return Err(ServerError::WrongNonce.http_status_400().into());
    }

    let redirect_to: String = Itertools::intersperse(
        vec!["User", &user.id.to_string()]
            .into_iter()
            .chain(path_iter),
        "/",
    )
    .collect();
    let redirect_to = format!("{}{}", redirect_to, request.query_string());
    st.core_service
        .send_request(request.method(), request_body, &redirect_to)
        .await
}
