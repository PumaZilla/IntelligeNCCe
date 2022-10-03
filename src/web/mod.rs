mod routes;
use crate::error::{Error, Result};

pub async fn start(
    cfg: crate::config::Config,
    db: std::sync::Arc<crate::database::Connection>,
) -> Result<()> {
    let addr = cfg.address.clone();

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(cfg.clone()))
            .app_data(actix_web::web::Data::new(db.clone()))
            .app_data(actix_web::web::Data::new(
                crate::database::graphql::build_schema(),
            ))
            .configure(routes::register)
    })
    .bind(&addr)
    .map_err(|_| Error::WebBindError(addr.to_string()))?
    .run()
    .await
    .map_err(|e| Error::WebRuntimeError(e.to_string()))
}
