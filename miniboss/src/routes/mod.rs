use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod auth;
mod error;
mod appdata;
mod redirect;
mod empty;
mod v1;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(web::scope("/api")
            .configure(v1::Router::configure)
        );
    }
}