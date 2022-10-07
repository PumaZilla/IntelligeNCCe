#[derive(Clone)]
pub struct Context {
    pub pool: std::sync::Arc<super::Connection>,
}
impl juniper::Context for Context {}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

pub type Schema = juniper::RootNode<'static, Query, Mutation, juniper::EmptySubscription<Context>>;
pub fn build_schema() -> Schema {
    log::trace!("building graphql schema");
    Schema::new(Query, Mutation, juniper::EmptySubscription::new())
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

pub struct Query;
#[juniper::graphql_object(Context = Context)]
impl Query {
    pub async fn health() -> bool {
        log::trace!("graphql query received: health");
        true
    }
    pub async fn api_version() -> &'static str {
        log::trace!("graphql query received: apiVersion");
        "1"
    }

    pub async fn event(ctx: &Context) -> juniper::FieldResult<Vec<super::models::event::Model>> {
        log::trace!("graphql query received: event");
        super::models::event::Model::read(ctx).await
    }
    pub async fn event_by_type(ctx: &Context, type_: super::models::event::Type) -> juniper::FieldResult<Vec<super::models::event::Model>> {
        log::trace!("graphql query received: eventByType");
        super::models::event::Model::filter_by_type(ctx, type_).await
    }

    pub async fn keyword(
        ctx: &Context,
    ) -> juniper::FieldResult<Vec<super::models::keyword::Model>> {
        log::trace!("graphql query received: keyword");
        super::models::keyword::Model::read(ctx).await
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

pub struct Mutation;
#[juniper::graphql_object(Context = Context)]
impl Mutation {
    pub async fn create_event(
        ctx: &Context,
        event: super::models::event::NewModel,
    ) -> juniper::FieldResult<super::models::event::Model> {
        log::trace!("graphql mutation received: createEvent");
        event.create(ctx).await
    }
    pub async fn delete_event(
        ctx: &Context,
        id: i32,
    ) -> juniper::FieldResult<super::models::event::Model> {
        log::trace!("graphql mutation received: deleteEvent");
        super::models::event::Model::delete(ctx, id).await
    }

    pub async fn create_keyword(
        ctx: &Context,
        keyword: super::models::keyword::NewModel,
    ) -> juniper::FieldResult<super::models::keyword::Model> {
        log::trace!("graphql mutation received: createKeyword");
        keyword.create(ctx).await
    }
    pub async fn update_keyword(
        ctx: &Context,
    ) -> juniper::FieldResult<Vec<super::models::keyword::Model>> {
        log::trace!("graphql mutation received: updateKeyword");
        super::models::keyword::Model::update(ctx).await
    }
    pub async fn delete_keyword(
        ctx: &Context,
        id: i32,
    ) -> juniper::FieldResult<super::models::keyword::Model> {
        log::trace!("graphql mutation received: deleteKeyword");
        super::models::keyword::Model::delete(ctx, id).await
    }
}
