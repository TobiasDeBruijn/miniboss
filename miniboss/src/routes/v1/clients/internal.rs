use actix_web::web;
use serde::Serialize;
use database::oauth2_client::OAuth2Client;
use crate::routes::appdata::WDatabase;
use crate::routes::error::{WebError, WebResult};

#[derive(Serialize)]
pub struct Response {
    client_id: String,
    redirect_uri: String,
}

pub async fn internal(database: WDatabase) -> WebResult<web::Json<Response>> {
    let client = OAuth2Client::list(&database)
        .await?
        .into_iter()
        .find(|c| c.is_internal)
        .ok_or(WebError::InvalidInternalState)?;

    Ok(web::Json(Response {
        client_id: client.client_id,
        redirect_uri: client.redirect_uri,
    }))
}