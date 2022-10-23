use crate::{
    database::{
        models::Keyword,
        schema::{events, events_keywords, keywords},
        DBConnection,
    },
    error::{Error, Result},
};
use chrono::NaiveDateTime;
use diesel::{
    backend::{Backend, RawValue},
    deserialize::{self, FromSql, Result as Deserialized},
    pg::Pg,
    serialize::{Output, Result as Serialized, ToSql},
    sql_types::Integer,
    AsExpression, ExpressionMethods, Insertable, OptionalExtension, QueryDsl, Queryable,
    RunQueryDsl,
};
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Clone, GraphQLObject)]
#[graphql(name = "Event")]
pub struct Model {
    pub id: i32,
    pub template: String,
    pub type_: Type,
    pub source: String,
    pub data: String,
    pub created_at: NaiveDateTime,
    pub keywords: Vec<Keyword>,
}
impl Queryable<events::SqlType, Pg> for Model {
    type Row = (i32, String, i32, String, String, NaiveDateTime);

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(Self {
            id: row.0,
            template: row.1,
            type_: row.2.into(),
            source: row.3,
            data: row.4,
            created_at: row.5,
            keywords: Vec::new(),
        })
    }
}
impl Model {
    pub fn get_keywords(&mut self, pool: &DBConnection) -> Result<()> {
        let mut conn = pool
            .get()
            .map_err(|e| Error::DatabasePoolError(e.to_string()))?;
        self.keywords = events_keywords::table
            .filter(events_keywords::columns::event.eq(self.id))
            .inner_join(keywords::table)
            .select(keywords::all_columns)
            .load::<Keyword>(&mut conn)
            .map_err(|e| Error::DatabaseExecutionError(e.to_string()))?;
        Ok(())
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(GraphQLInputObject, Insertable)]
#[graphql(name = "NewEvent")]
#[diesel(table_name = events)]
pub struct NewModel {
    pub template: String,
    pub type_: Type,
    pub source: String,
    pub data: String,
}
impl NewModel {
    pub fn save_into_db(&self, pool: &DBConnection, keyword: i32) -> Result<Model> {
        let mut conn = pool
            .get()
            .map_err(|e| Error::DatabasePoolError(e.to_string()))?;
        let mut event_exists = events::table
            // check if the event already exists
            .filter(events::columns::source.eq(&self.source))
            .filter(events::columns::data.eq(&self.data))
            .first::<Model>(&mut conn)
            .optional()
            .map_err(|e| Error::DatabaseExecutionError(e.to_string()))?;
        // if not exists, insert it
        if event_exists.is_none() {
            event_exists = diesel::insert_into(events::table)
                .values(self)
                .get_result::<Model>(&mut conn)
                .optional()
                .map_err(|e| Error::DatabaseExecutionError(e.to_string()))?;
        }
        let event = event_exists.ok_or(Error::DatabaseExecutionError(format!(
            "Error inserting event: {}::{}",
            self.template, self.source
        )))?;
        // create the link between the event and the keyword
        let assoc_exists = events_keywords::table
            // check if the association between the event and the keyword already exists
            .filter(events_keywords::columns::event.eq(event.id))
            .filter(events_keywords::columns::keyword.eq(keyword))
            .first::<AssociationKeyword>(&mut conn)
            .optional()
            .map_err(|e| Error::DatabaseExecutionError(e.to_string()))?;
        // if not exists, insert it
        if assoc_exists.is_none() {
            diesel::insert_into(events_keywords::table)
                .values(AssociationKeyword::new(event.id, keyword))
                .execute(&mut conn)
                .map_err(|e| Error::DatabaseExecutionError(e.to_string()))?;
        }
        // return the event
        Ok(event)
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Clone, Debug, Default, PartialEq, GraphQLEnum, AsExpression)]
#[graphql(name = "EventType")]
#[diesel(sql_type = Integer)]
pub enum Type {
    #[default]
    Info,
    Paste,
    Blacklist,
}
impl From<i32> for Type {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Paste,
            2 => Self::Blacklist,
            _ => Self::Info,
        }
    }
}
impl From<&str> for Type {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "paste" => Self::Paste,
            "blacklist" => Self::Blacklist,
            _ => Self::Info,
        }
    }
}
impl From<String> for Type {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}
impl<DB> ToSql<Integer, DB> for Type
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> Serialized {
        match self {
            Self::Info => 0.to_sql(out),
            Self::Paste => 1.to_sql(out),
            Self::Blacklist => 2.to_sql(out),
        }
    }
}
impl<DB> FromSql<Integer, DB> for Type
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(value: RawValue<DB>) -> Deserialized<Self> {
        let x = i32::from_sql(value)?;
        match x {
            0..=1 => Ok(x.into()),
            _ => Err(format!("unrecognized enum variant: {}", x).into()),
        }
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Insertable, Queryable)]
#[diesel(table_name = events_keywords)]
struct AssociationKeyword {
    pub event: i32,
    pub keyword: i32,
}
impl AssociationKeyword {
    pub fn new(event: i32, keyword: i32) -> Self {
        Self { event, keyword }
    }
}
