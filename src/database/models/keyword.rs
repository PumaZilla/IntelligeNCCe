use crate::error::{Error, Result};

#[derive(
    Clone, Debug, PartialEq, serde::Deserialize, juniper::GraphQLObject, diesel::Queryable,
)]
#[graphql(name = "Keyword")]
#[diesel(table_name = crate::database::schema::keyword)]
pub struct Model {
    /// The keyword's ID
    pub id: i32,
    /// Type of the keyword
    pub type_: String,
    /// The keyword
    pub value: String,
    /// The date when the keyword was created
    pub created_at: chrono::NaiveDateTime,
    /// The date when the keyword was last consulted
    pub last_consulted: chrono::NaiveDateTime,
}
impl Model {
    pub async fn all(pool: &crate::database::Connection) -> Result<Vec<Model>> {
        use diesel::RunQueryDsl;
        let mut client = pool
            .get()
            .map_err(|e| Error::DatabasePoolError(e.to_string()))?;
        Ok(crate::database::schema::keyword::table
            .load::<Model>(&mut client)
            .map_err(|e| Error::DatabaseReadError(e.to_string()))?)
    }

    pub async fn read(ctx: &crate::database::graphql::Context) -> juniper::FieldResult<Vec<Self>> {
        Ok(Self::all(&ctx.pool).await?)
    }

    pub async fn update(
        ctx: &crate::database::graphql::Context,
    ) -> juniper::FieldResult<Vec<Self>> {
        use diesel::{ExpressionMethods, RunQueryDsl};
        let mut client = ctx.pool.get()?;
        Ok(diesel::update(crate::database::schema::keyword::table)
            .set(
                crate::database::schema::keyword::last_consulted.eq(chrono::Utc::now().naive_utc()),
            )
            .get_results(&mut client)?)
    }

    pub async fn delete(
        ctx: &crate::database::graphql::Context,
        id: i32,
    ) -> juniper::FieldResult<Model> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let mut client = ctx.pool.get()?;
        Ok(diesel::delete(
            crate::database::schema::keyword::table
                .filter(crate::database::schema::keyword::dsl::id.eq(id)),
        )
        .get_result(&mut client)?)
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(
    Clone, Debug, PartialEq, serde::Deserialize, juniper::GraphQLInputObject, diesel::Insertable,
)]
#[graphql(name = "NewKeyword")]
#[diesel(table_name = crate::database::schema::keyword)]
pub struct NewModel {
    /// Type of the keyword
    pub type_: String,
    /// The keyword
    pub value: String,
}
impl NewModel {
    pub async fn save(
        &self,
        pool: &crate::database::Connection,
    ) -> std::result::Result<Model, Box<dyn std::error::Error>> {
        // FIXME: proper error
        use diesel::RunQueryDsl;
        let mut client = pool.get()?;
        Ok(diesel::insert_into(crate::database::schema::keyword::table)
            .values(self)
            .get_result(&mut client)?)
    }

    pub async fn create(
        &self,
        ctx: &crate::database::graphql::Context,
    ) -> juniper::FieldResult<Model> {
        Ok(self.save(&ctx.pool).await?)
    }
}
