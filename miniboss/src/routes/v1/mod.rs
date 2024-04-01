use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod oauth;
mod user;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(web::scope("/v1")
            .configure(oauth::Router::configure)
            .configure(user::Router::configure)
        );
    }
}