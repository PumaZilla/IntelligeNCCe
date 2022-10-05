mod routes;

pub async fn start(cfg: crate::config::Config, db: std::sync::Arc<crate::database::Connection>) {
    let addr = cfg.address.clone();
    let srv = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(cfg.clone()))
            .app_data(actix_web::web::Data::new(db.clone()))
            .app_data(actix_web::web::Data::new(
                crate::database::graphql::build_schema(),
            ))
            .configure(routes::register)
    })
    .bind(&addr);
    match srv {
        Ok(srv) => {
            log::info!("starting web server on {}...", &addr);
            if let Err(_) = srv.run().await {
                log::error!("{}", crate::error::Error::WebRuntimeError(addr.to_string()));
            }
        }
        Err(_) => log::error!("{}", crate::error::Error::WebBindError(addr.to_string())),
    }
}
