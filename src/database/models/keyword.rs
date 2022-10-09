use crate::{
    database::{models::Event, schema::keywords, DBConnection},
    error::{Error, Result},
};
use chrono::NaiveDateTime;
use diesel::{
    backend::{Backend, RawValue},
    deserialize::{self, FromSql, Result as Deserialized},
    pg::Pg,
    serialize::{Output, Result as Serialized, ToSql},
    sql_types::Integer,
    AsExpression, ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable,
};
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Clone, GraphQLObject)]
#[graphql(name = "Keyword")]
pub struct Model {
    pub id: i32,
    pub type_: Type,
    pub value: String,
    pub created_at: NaiveDateTime,
    pub last_consulted: NaiveDateTime,
    pub events: Vec<Event>,
}
impl Queryable<keywords::SqlType, Pg> for Model {
    type Row = (i32, i32, String, NaiveDateTime, NaiveDateTime);

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(Self {
            id: row.0,
            type_: row.1.into(),
            value: row.2,
            created_at: row.3,
            last_consulted: row.4,
            events: Vec::new(),
        })
    }
}
impl<DB> Selectable<DB> for Model
where
    DB: Backend,
{
    type SelectExpression = (
        keywords::id,
        keywords::type_,
        keywords::value,
        keywords::created_at,
        keywords::last_consulted,
    );

    fn construct_selection() -> Self::SelectExpression {
        (
            keywords::id,
            keywords::type_,
            keywords::value,
            keywords::created_at,
            keywords::last_consulted,
        )
    }
}
impl Model {
    pub fn get_all(pool: &DBConnection) -> Result<Vec<Model>> {
        let mut conn = pool
            .get()
            .map_err(|e| Error::DatabasePoolError(e.to_string()))?;
        Ok(keywords::table
            .load(&mut conn)
            .map_err(|e| Error::DatabaseExecutionError(e.to_string()))?)
    }
    pub fn get_events(&mut self, pool: &DBConnection) -> Result<()> {
        use crate::database::schema::{events, events_keywords};
        let mut conn = pool
            .get()
            .map_err(|e| Error::DatabasePoolError(e.to_string()))?;
        self.events = events_keywords::table
            .filter(events_keywords::columns::keyword.eq(self.id))
            .inner_join(events::table)
            .select(events::all_columns)
            .load::<Event>(&mut conn)
            .map_err(|e| Error::DatabaseExecutionError(e.to_string()))?;
        Ok(())
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(GraphQLInputObject, Insertable)]
#[graphql(name = "NewKeyword")]
#[diesel(table_name = keywords)]

pub struct NewModel {
    pub type_: Type,
    pub value: String,
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Clone, Debug, Default, PartialEq, GraphQLEnum, AsExpression)]
#[graphql(name = "KeywordType")]
#[diesel(sql_type = Integer)]
pub enum Type {
    #[default]
    Text,
    Ip,
    Domain,
    Url,
    Email,
    Credential,
}
impl From<i32> for Type {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Ip,
            2 => Self::Domain,
            3 => Self::Url,
            4 => Self::Email,
            5 => Self::Credential,
            _ => Self::Text,
        }
    }
}
impl<DB> ToSql<Integer, DB> for Type
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> Serialized {
        match self {
            Self::Ip => 1.to_sql(out),
            Self::Domain => 2.to_sql(out),
            Self::Url => 3.to_sql(out),
            Self::Email => 4.to_sql(out),
            Self::Credential => 5.to_sql(out),
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
            0 => Ok(Self::Text),
            1 => Ok(Self::Ip),
            2 => Ok(Self::Domain),
            3 => Ok(Self::Url),
            4 => Ok(Self::Email),
            5 => Ok(Self::Credential),
            _ => Err(format!("unrecognized enum variant: {}", x).into()),
        }
    }
}
