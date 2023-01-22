use crate::{
    database::user::User,
    error::{ResponseError, ServerError},
    rest_api::{decode_rsa_parameter, into_success_response},
    server_state::ServerState,
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub login: String,
    pub email: String,    // RSA-OAEP(email), base58, OAEP SHA-256 padding
    pub password: String, // RSA-OAEP(SHA-256(password), base58), base58, OAEP SHA-256 padding
}

#[derive(Serialize, Deserialize)]
pub struct Response {}
into_success_response!(Response);

pub async fn execute(
    st: web::Data<ServerState>,
    data: web::Json<Request>,
) -> actix_web::Result<HttpResponse> {
    let email = decode_rsa_parameter(&data.email, "email".to_string(), &st)?;
    let password = decode_rsa_parameter(&data.password, "password".to_string(), &st)?;

    let user = User {
        login: data.login.clone(),
        email,
        password,
    };

    let inserted = user
        .try_insert(&st.db_connection.pool)
        .await
        .map_err(|e| e.http_status_500())?;

    if inserted {
        Response {}.into()
    } else {
        Err(ServerError::RegisterFailed.http_status_400().into())
    }
}
