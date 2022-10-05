#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Deserialize,
    juniper::GraphQLObject,
    diesel::Queryable,
    diesel::Selectable,
)]
#[graphql(name = "Event")]
#[diesel(table_name = crate::database::schema::event)]
pub struct Model {
    /// The event's ID
    pub id: i32,
    /// Template who gather the information
    pub source: String,
    /// Event type of the event
    pub type_: String,
    /// The website or server where the data was stored
    pub location: String,
    /// The found data
    pub data: String,
}
impl Model {
    pub async fn read(ctx: &crate::database::graphql::Context) -> juniper::FieldResult<Vec<Self>> {
        use diesel::RunQueryDsl;
        let mut client = ctx.pool.get()?;
        Ok(crate::database::schema::event::dsl::event.load::<Self>(&mut client)?)
    }

    pub async fn delete(
        ctx: &crate::database::graphql::Context,
        id: i32,
    ) -> juniper::FieldResult<Model> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let mut client = ctx.pool.get()?;
        Ok(diesel::delete(
            crate::database::schema::event::dsl::event
                .filter(crate::database::schema::event::dsl::id.eq(id)),
        )
        .get_result(&mut client)?)
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(
    Clone, Debug, PartialEq, serde::Deserialize, juniper::GraphQLInputObject, diesel::Insertable,
)]
#[graphql(name = "NewEvent")]
#[diesel(table_name = crate::database::schema::event)]
pub struct NewModel {
    pub source: String,
    pub type_: String,
    pub location: String,
    pub data: String,
}
impl NewModel {
    pub async fn save(
        &self,
        pool: &crate::database::Connection,
    ) -> Result<Model,Box<dyn std::error::Error>> {
        use diesel::RunQueryDsl;
        let mut client = pool.get()?;
        Ok(
            diesel::insert_into(crate::database::schema::event::dsl::event)
                .values(self)
                .get_result(&mut client)?,
        )
    }

    pub async fn create(
        &self,
        ctx: &crate::database::graphql::Context,
    ) -> juniper::FieldResult<Model> {
        Ok(self.save(&ctx.pool).await?)
    }
}
