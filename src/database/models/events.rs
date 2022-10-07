use crate::{
    database::{models::Keyword, schema::events, DBConnection},
    error::{Error, Result},
};
use chrono::NaiveDateTime;
use diesel::{
    deserialize, pg::Pg, ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl,
};
use juniper::{GraphQLInputObject, GraphQLObject};

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Clone, GraphQLObject)]
#[graphql(name = "Event")]
pub struct Model {
    pub id: i32,
    pub template: String,
    pub type_: String,
    pub source: String,
    pub data: String,
    pub created_at: NaiveDateTime,
    pub keywords: Vec<Keyword>,
}
impl Queryable<events::SqlType, Pg> for Model {
    type Row = (i32, String, String, String, String, NaiveDateTime);

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(Self {
            id: row.0,
            template: row.1,
            type_: row.2,
            source: row.3,
            data: row.4,
            created_at: row.5,
            keywords: Vec::new(),
        })
    }
}
impl Model {
    pub fn get_keywords(&mut self, pool: &DBConnection) -> Result<()> {
        /*
        use crate::database::schema::{events_keywords, keywords};
        let mut conn = pool
            .get()
            .map_err(|e| Error::DatabasePoolError(e.to_string()))?;
        events_keywords::table
            .left_join(keywords::table)
            .filter(events_keywords::columns::event.eq(self.id))
            .select(keywords::columns::id)
            .load(&mut conn)
            .map_err(|e| Error::DatabaseExecutionError(e.to_string()))?
            .into_iter()
            .for_each(|keyword| self.keywords.push(keyword));
            */
        unimplemented!()
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(GraphQLInputObject, Insertable)]
#[graphql(name = "NewEvent")]
#[diesel(table_name = events)]

pub struct NewModel {
    pub template: String,
    pub type_: String,
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