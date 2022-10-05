pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        // Common routes
        .service(actix_web::web::resource("/").route(actix_web::web::get().to(index)))
        // GraphQL routes
        .service(
            actix_web::web::resource(GRAPHQL_ENDPOINT).route(actix_web::web::post().to(graphql)),
        )
        .service(
            actix_web::web::resource("/i_understand_that_this_is_against_security")
                .route(actix_web::web::get().to(graphiql)),
        );
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

async fn index() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("Hello world!") // FIXME: return a proper index page
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

const GRAPHQL_ENDPOINT: &str = "/graphql";

async fn graphql(
    req: actix_web::web::Json<juniper::http::GraphQLRequest>,
    schema: actix_web::web::Data<crate::database::graphql::Schema>,
    db: actix_web::web::Data<std::sync::Arc<crate::database::Connection>>,
) -> impl actix_web::Responder {
    log::trace!("new graphql request received.");
    let ctx = crate::database::graphql::Context {
        pool: db.get_ref().clone(),
    };
    let res = req.execute(&schema, &ctx).await;
    actix_web::HttpResponse::Ok().json(res)
}

async fn graphiql(
    req: actix_web::HttpRequest,
    cfg: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    let ip = req
        .peer_addr()
        .map(|x| x.to_string())
        .unwrap_or("someone".to_string());
    if !cfg.insecure {
        log::warn!("{} tried to access the graphiql endpoint.", ip);
        return actix_web::HttpResponse::NotFound().finish();
    }
    log::warn!("{} has been access the graphiql endpoint.", ip);
    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(juniper::http::graphiql::graphiql_source(
            GRAPHQL_ENDPOINT,
            None,
        ))
}
