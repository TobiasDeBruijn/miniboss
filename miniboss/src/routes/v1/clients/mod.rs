use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod internal;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(web::scope("/clients")
            .route("/internal", web::get().to(internal::internal))
        );
    }
}