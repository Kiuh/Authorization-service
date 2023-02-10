use crate::{
    database::user::UserData,
    error::{IntoHttpResult, ServerError},
    rest_api::{into_success_response, RsaEncodedParameter},
    server_state::ServerState,
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub login: String,
    pub email: RsaEncodedParameter,
    pub password: RsaEncodedParameter,
}

#[derive(Serialize, Deserialize)]
pub struct Response {}
into_success_response!(Response);

pub async fn execute(
    st: web::Data<ServerState>,
    data: web::Json<Request>,
) -> actix_web::Result<HttpResponse> {
    let email = data.email.decode("email".to_string(), &st)?;
    let password = data.password.decode("password".to_string(), &st)?;

    let user = UserData {
        login: data.login.clone(),
        email,
        password,
    };

    let inserted = user
        .try_insert(&st.db_connection.pool)
        .await
        .into_http_500()?;

    if inserted {
        Response {}.into()
    } else {
        Err(ServerError::RegisterFailed).into_http_400()
    }
}
