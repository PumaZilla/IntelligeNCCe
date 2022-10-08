use crate::{
    database::{schema::keywords, DBConnection},
    error::{Error, Result},
};
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, RunQueryDsl, Selectable};
use juniper::{GraphQLInputObject, GraphQLObject};

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Clone, GraphQLObject, Queryable, Selectable)]
#[graphql(name = "Keyword")]
#[diesel(table_name = keywords)]
pub struct Model {
    pub id: i32,
    pub type_: String,
    pub value: String,
    pub created_at: NaiveDateTime,
    pub last_consulted: NaiveDateTime,
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
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(GraphQLInputObject, Insertable)]
#[graphql(name = "NewKeyword")]
#[diesel(table_name = keywords)]

pub struct NewModel {
    pub type_: String,
    pub value: String,
}
