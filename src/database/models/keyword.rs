#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Deserialize,
    juniper::GraphQLObject,
    diesel::Queryable,
    diesel::Selectable,
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
    pub async fn read(ctx: &crate::database::graphql::Context) -> juniper::FieldResult<Vec<Self>> {
        use diesel::RunQueryDsl;
        let mut client = ctx.pool.get()?;
        Ok(crate::database::schema::keyword::table.load::<Self>(&mut client)?)
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
    ) -> Result<Model, Box<dyn std::error::Error>> {
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
