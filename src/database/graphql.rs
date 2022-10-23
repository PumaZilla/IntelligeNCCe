use super::models::{Secret, Event, Keyword, NewSecret, NewEvent, NewKeyword};
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use juniper::{graphql_object, EmptySubscription, FieldResult, RootNode};
use std::sync::Arc;

#[derive(Clone)]
pub struct Context {
    pub pool: Arc<super::DBConnection>,
}
impl Context {
    pub fn new(pool: Arc<super::DBConnection>) -> Self {
        Self { pool }
    }
}
impl juniper::Context for Context {}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;
pub fn build_schema() -> Schema {
    log::trace!("building graphql schema");
    Schema::new(Query, Mutation, EmptySubscription::new())
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

pub struct Query;
#[graphql_object(Context = Context)]
impl Query {
    pub fn health() -> bool {
        log::trace!("graphql query received: health");
        true
    }
    pub fn api_version() -> &'static str {
        log::trace!("graphql query received: apiVersion");
        "1"
    }
    pub fn app_version() -> &'static str {
        log::trace!("graphql query received: appVersion");
        std::env!("CARGO_PKG_VERSION")
    }

    pub fn secrets(ctx: &Context) -> FieldResult<Vec<Secret>> {
        log::trace!("graphql query received: secrets");
        use super::schema::secrets;
        let mut conn = ctx.pool.get()?;
        Ok(secrets::table.load::<Secret>(&mut conn)?)
    }

    pub fn event(ctx: &Context, id: i32) -> FieldResult<Event> {
        log::trace!("graphql query received: event");
        use super::schema::events;
        let mut conn = ctx.pool.get()?;
        let mut event = events::table
            .filter(events::columns::id.eq(id))
            .first::<Event>(&mut conn)?;
        event.get_keywords(&ctx.pool)?;
        Ok(event)
    }
    pub fn events(ctx: &Context) -> FieldResult<Vec<Event>> {
        log::trace!("graphql query received: events");
        use super::schema::events;
        let mut conn = ctx.pool.get()?;
        let mut events = events::table.load::<Event>(&mut conn)?;
        events.iter_mut().for_each(|event| {
            if let Err(err) = event.get_keywords(&ctx.pool) {
                log::error!("failed to get keywords for event {}: {}", event.id, err);
            }
        });
        Ok(events)
    }
    
    pub fn keyword(ctx: &Context, id: i32) -> FieldResult<Keyword> {
        log::trace!("graphql query received: keyword");
        use super::schema::keywords;
        let mut conn = ctx.pool.get()?;
        let mut keyword = keywords::table
            .filter(keywords::columns::id.eq(id))
            .first::<Keyword>(&mut conn)?;
        keyword.get_events(&ctx.pool)?;
        Ok(keyword)
    }
    pub fn keywords(ctx: &Context) -> FieldResult<Vec<Keyword>> {
        log::trace!("graphql query received: keywords");
        let mut keywords = Keyword::get_all(&ctx.pool)?;
        keywords.iter_mut().for_each(|keyword| {
            if let Err(err) = keyword.get_events(&ctx.pool) {
                log::error!("failed to get events for keyword {}: {}", keyword.id, err);
            }
        });
        Ok(keywords)
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

pub struct Mutation;
#[graphql_object(Context = Context)]
impl Mutation {
    pub fn create_secret(ctx: &Context, secret: NewSecret) -> FieldResult<Secret> {
        log::trace!("graphql mutation received: createSecret");
        use crate::database::schema::secrets;
        let mut conn = ctx.pool.get()?;
        Ok(diesel::insert_into(secrets::table)
            .values(secret)
            .get_result(&mut conn)?)
    }
    pub fn create_event(ctx: &Context, keyword_id: i32, event: NewEvent) -> FieldResult<Event> {
        log::trace!("graphql mutation received: createEvent");
        Ok(event.save_into_db(&ctx.pool, keyword_id)?)
    }
    pub fn create_keyword(ctx: &Context, keyword: NewKeyword) -> FieldResult<Keyword> {
        log::trace!("graphql mutation received: createKeyword");
        use crate::database::schema::keywords;
        let mut conn = ctx.pool.get()?;
        Ok(diesel::insert_into(keywords::table)
            .values(keyword)
            .get_result(&mut conn)?)
    }

    /*
    pub fn create_event(ctx: &Context, event: NewEvent) -> FieldResult<Event> {
        log::trace!("graphql mutation received: createEvent");
        event.create(ctx).await
    }
    pub fn delete_event(ctx: &Context, id: i32) -> FieldResult<Event> {
        log::trace!("graphql mutation received: deleteEvent");
        Event::delete(ctx, id).await
    }

    pub fn create_keyword(ctx: &Context, keyword: NewKeyword) -> FieldResult<Keyword> {
        log::trace!("graphql mutation received: createKeyword");
        keyword.create(ctx).await
    }
    pub fn update_keyword(ctx: &Context) -> FieldResult<Vec<Keyword>> {
        log::trace!("graphql mutation received: updateKeyword");
        Keyword::update(ctx).await
    }
    pub fn delete_keyword(ctx: &Context, id: i32) -> FieldResult<Keyword> {
        log::trace!("graphql mutation received: deleteKeyword");
        Keyword::delete(ctx, id).await
    }
    */
}
