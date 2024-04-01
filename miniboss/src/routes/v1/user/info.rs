use actix_web::web;

use database::user::User;
use crate::routes::auth::Auth;

pub async fn info(auth: Auth) -> web::Json<User> {
    web::Json(auth.user)
}