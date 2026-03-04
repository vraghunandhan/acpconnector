use actix_web::web;

pub mod delegate_payment;
pub mod validate_payment;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/agentic_commerce")
            .route("/delegate_payment", web::post().to(delegate_payment::delegate_payment))
            .route("/validate_payment", web::post().to(validate_payment::validate_payment)),
    );
}
