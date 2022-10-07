#[derive(rust_embed::RustEmbed)]
#[folder = "www/dist/"]
struct Assets;
impl Assets {
    fn handle(path: &str) -> actix_web::HttpResponse {
        match Assets::get(path) {
            Some(content) => actix_web::HttpResponse::Ok()
                .content_type(mime_guess::from_path(path).first_or_octet_stream().as_ref())
                .body(content.data.into_owned()),
            None => actix_web::HttpResponse::NotFound().finish(),
        }
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        // Common routes
        .service(actix_web::web::resource("/").route(actix_web::web::get().to(index)))
        .service(actix_web::web::resource("/assets/{_:.*}").route(actix_web::web::get().to(assets)))
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

async fn index() -> actix_web::HttpResponse {
    Assets::handle("index.html")
}

async fn assets(path: actix_web::web::Path<String>) -> actix_web::HttpResponse {
    Assets::handle(["assets", path.as_str()].join("/").as_str())
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

const GRAPHQL_ENDPOINT: &str = "/graphql";

async fn graphql(
    req: actix_web::HttpRequest,
    gqlreq: actix_web::web::Json<juniper::http::GraphQLRequest>,
    schema: actix_web::web::Data<crate::database::graphql::Schema>,
    db: actix_web::web::Data<std::sync::Arc<crate::database::DBConnection>>,
) -> impl actix_web::Responder {
    log::trace!(
        "new graphql request received from {}.",
        req.peer_addr()
            .map(|x| x.to_string())
            .unwrap_or("someone".to_string())
    );
    let ctx = crate::database::graphql::Context::new(db.get_ref().clone());
    let res = gqlreq.execute(&schema, &ctx).await;
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
