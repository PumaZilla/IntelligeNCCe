use crate::{
    database::{schema::secrets, DBConnection},
    error::{Error, Result},
};
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, QueryDsl, RunQueryDsl, Queryable};
use juniper::{GraphQLInputObject, GraphQLObject};

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Clone, Queryable, GraphQLObject)]
#[graphql(name = "Secret")]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub value: String,
    pub created_at: NaiveDateTime,
}
impl Model {
    pub fn get_secret_by_name(&mut self, pool: &DBConnection, name: &str) -> Result<Option<Self>> {
        let mut conn = pool
            .get()
            .map_err(|e| Error::DatabasePoolError(e.to_string()))?;
        Ok(secrets::table
            .filter(secrets::columns::name.eq(name))
            .limit(1)
            .load::<Self>(&mut conn)
            .map_err(|e| Error::DatabaseExecutionError(e.to_string()))?
            .pop())
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(GraphQLInputObject, Insertable)]
#[graphql(name = "NewSecret")]
#[diesel(table_name = secrets)]
pub struct NewModel {
    pub name: String,
    pub value: String,
}
