use crate::{
    database::{models::Event, schema::keywords, DBConnection},
    error::{Error, Result},
};
use chrono::NaiveDateTime;
use diesel::{
    backend::Backend, deserialize, pg::Pg, ExpressionMethods, Insertable, QueryDsl, Queryable,
    RunQueryDsl, Selectable,
};
use juniper::{GraphQLInputObject, GraphQLObject};

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Clone, GraphQLObject)]
#[graphql(name = "Keyword")]
pub struct Model {
    pub id: i32,
    pub type_: String,
    pub value: String,
    pub created_at: NaiveDateTime,
    pub last_consulted: NaiveDateTime,
    pub events: Vec<Event>,
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
impl Queryable<keywords::SqlType, Pg> for Model {
    type Row = (i32, String, String, NaiveDateTime, NaiveDateTime);

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(Self {
            id: row.0,
            type_: row.1,
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

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(GraphQLInputObject, Insertable)]
#[graphql(name = "NewKeyword")]
#[diesel(table_name = keywords)]

pub struct NewModel {
    pub type_: String,
    pub value: String,
}
