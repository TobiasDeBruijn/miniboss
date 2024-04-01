use actix_web::web;
use serde::{Deserialize, Serialize};
use database::user::User;
use crate::routes::appdata::{WConfig, WDatabase};
use crate::routes::error::{WebError, WebResult};

#[derive(Deserialize)]
pub struct Request {
    name: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct Response {
    id: String,
}

pub async fn register(
    database: WDatabase,
    config: WConfig,
    payload: web::Json<Request>
) -> WebResult<web::Json<Response>> {
    let payload = payload.into_inner();
    if User::get_by_email(&database, &payload.email).await?.is_some() {
        return Err(WebError::BadRequest);
    }

    let total_user_count = User::list(&database).await?.len();

    let user = User::new(
        &database,
        payload.name,
        payload.email,
        total_user_count == 0
    ).await?;

    user.set_password(&payload.password, &config.password_pepper, &database).await?;

    Ok(web::Json(Response {
        id: user.user_id,
    }))
}