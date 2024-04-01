use crate::routes::auth::Auth;
use crate::routes::error::WebResult;
use actix_web::web;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    scope: String,
}

pub async fn token_info(auth: Auth) -> WebResult<web::Json<Response>> {
    Ok(web::Json(Response {
        scope: auth.scopes().into_iter().collect::<Vec<_>>().join(" "),
    }))
}