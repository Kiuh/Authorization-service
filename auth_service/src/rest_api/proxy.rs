use actix_web::{web, HttpRequest, HttpResponse};
use itertools::Itertools;

use crate::{
    error::{IntoHttpResult, ServerError},
    server_state::ServerState,
};

use super::authorize::authorize;

pub async fn execute(
    st: web::Data<ServerState>,
    request: HttpRequest,
    request_body: actix_web::web::Bytes,
) -> actix_web::Result<HttpResponse> {
    let mut path_iter = request.path().split('/');
    let (_, user_literal, login) = (path_iter.next(), path_iter.next(), path_iter.next());
    if Some("User") != user_literal || login.is_none() {
        return Err(ServerError::WrongRequest).into_http_404();
    }
    let login = login.unwrap();

    let user = authorize(login, &request, &st.db_connection.pool).await?;

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
