use crate::{
    database::{models::Keyword, schema::events, DBConnection},
    error::{Error, Result},
};
use chrono::NaiveDateTime;
use diesel::{
    backend::{Backend, RawValue},
    deserialize::{self, FromSql, Result as Deserialized},
    pg::Pg,
    serialize::{Output, Result as Serialized, ToSql},
    sql_types::Integer,
    AsExpression, ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl,
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
        use crate::database::schema::{events_keywords, keywords};
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
    pub fn save_into_db(&self, pool: &DBConnection) -> Result<Model> {
        let mut conn = pool
            .get()
            .map_err(|e| Error::DatabasePoolError(e.to_string()))?;
        Ok(diesel::insert_into(events::table)
            .values(self)
            .get_result(&mut conn)
            .map_err(|e| Error::DatabaseExecutionError(e.to_string()))?)
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Clone, Debug, Default, PartialEq, GraphQLEnum, AsExpression)]
#[graphql(name = "EventType")]
#[diesel(sql_type = Integer)]
pub enum Type {
    #[default]
    Paste,
}
impl From<i32> for Type {
    fn from(value: i32) -> Self {
        match value {
            _ => Self::Paste,
        }
    }
}
impl From<&str> for Type {
    fn from(value: &str) -> Self {
        match value {
            _ => Self::Paste,
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
            _ => 0.to_sql(out),
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
            0 => Ok(Self::Paste),
            _ => Err(format!("unrecognized enum variant: {}", x).into()),
        }
    }
}
