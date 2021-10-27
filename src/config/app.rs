use crate::api::*;
use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");
    cfg.service(
        web::scope("/api")
            // .service(ping_controller::ping)
            .service(
                web::scope("/auth")
                    .service(
                web::resource("/signup").route(web::post().to(account_controller::signup)),
                    )
                    .service(
                web::resource("/login").route(web::post().to(account_controller::login)),
                    )
                    .service(
                web::resource("/logout").route(web::post().to(account_controller::logout)),
                    ),
            )
            .service(
                web::scope("/user")
                    .service(
                web::resource("")
                        .route(web::get().to(account_controller::find_all)),
                    )
                    .service(
                web::resource("/{id}")
                        .route(web::get().to(account_controller::find_by_id))
                        .route(web::put().to(account_controller::update))
                        .route(web::delete().to(account_controller::delete)),
                    )
            )
    );
}
