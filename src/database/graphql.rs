#[derive(Clone)]
pub struct Context {
    pub pool: std::sync::Arc<super::Connection>,
}
impl juniper::Context for Context {}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

pub type Schema = juniper::RootNode<'static, Query, Mutation, juniper::EmptySubscription<Context>>;
pub fn build_schema() -> Schema {
    Schema::new(Query, Mutation, juniper::EmptySubscription::new())
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

pub struct Query;
#[juniper::graphql_object(Context = Context)]
impl Query {
    pub async fn health() -> bool {
        true
    }
    pub async fn api_version() -> &'static str {
        "1"
    }

    pub async fn object(ctx: &Context) -> juniper::FieldResult<Vec<super::models::object::Model>> {
        super::models::object::Model::read(ctx).await
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

pub struct Mutation;
#[juniper::graphql_object(Context = Context)]
impl Mutation {
    pub async fn create_object(
        ctx: &Context,
        object: super::models::object::NewModel,
    ) -> juniper::FieldResult<super::models::object::Model> {
        object.create(ctx).await
    }

    pub async fn delete_object(
        ctx: &Context,
        id: i32,
    ) -> juniper::FieldResult<super::models::object::Model> {
        super::models::object::Model::delete(ctx, id).await
    }
}
