use crate::routes::appdata::{WConfig, WDatabase};
use crate::routes::error::{WebError, WebResult};
use actix_web::web;
use database::oauth2_client::OAuth2PendingAuthorization;
use database::user::User;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Deserialize)]
pub struct Request {
    authorization: String,
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct Response {
    status: bool,
}

pub async fn login(
    database: WDatabase,
    config: WConfig,
    payload: web::Json<Request>,
) -> WebResult<web::Json<Response>> {
    let authorization = OAuth2PendingAuthorization::get_by_id(&database, &payload.authorization)
        .await?
        .ok_or(WebError::NotFound)?;

    let user = User::get_by_email(&database, &payload.username).await?
        .ok_or(WebError::Unauthorized)?;

    if !user.verify_password(&payload.password, &config.password_pepper, &database).await? {
        return Err(WebError::Unauthorized)
    }

    // OAuth2 defines `scope` to be all scopes, seperated by a ' ' (space char)
    // Where duplicates can be ignored.
    let scope_set = authorization
        .scopes()
        .clone()
        .map(|s| s.split(" ").map(|c| c.to_string()).collect::<HashSet<_>>())
        .unwrap_or_default();

    if !user.is_admin {
        let permitted_scopes =
            HashSet::from_iter(user.list_permitted_scopes(&database).await?);

        let oidc_scopes = oidc_scopes();
        let allowed_scopes = permitted_scopes
            .union(&oidc_scopes)
            .map(|c| c.to_string())
            .collect::<HashSet<_>>();

        let disallowed_scopes = scope_set
            .difference(&allowed_scopes)
            .collect::<HashSet<_>>();

        if !disallowed_scopes.is_empty() {
            return Err(WebError::Forbidden);
        }
    }

    authorization
        .set_user_id(&database, &user.user_id)
        .await
        .map_err(|_| WebError::BadRequest)?;

    Ok(web::Json(Response {
        status: true,
    }))
}

fn oidc_scopes() -> HashSet<String> {
    HashSet::from_iter([
        "openid".to_string(),
        "profile".to_string(),
        "email".to_string(),
    ])
}