#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Deserialize,
    juniper::GraphQLObject,
    diesel::Queryable,
    diesel::Selectable,
)]
#[graphql(name = "Object")]
#[diesel(table_name = crate::database::schema::object)]
pub struct Model {
    /// The object's ID
    pub id: i32,
    /// Template who gather the information
    pub source: String,
    /// Event type of the object
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
        Ok(crate::database::schema::object::dsl::object.load::<Self>(&mut client)?)
    }

    pub async fn delete(
        ctx: &crate::database::graphql::Context,
        id: i32,
    ) -> juniper::FieldResult<Model> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let mut client = ctx.pool.get()?;
        Ok(diesel::delete(
            crate::database::schema::object::dsl::object
                .filter(crate::database::schema::object::dsl::id.eq(id)),
        )
        .get_result(&mut client)?)
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(
    Clone, Debug, PartialEq, serde::Deserialize, juniper::GraphQLInputObject, diesel::Insertable,
)]
#[graphql(name = "NewObject")]
#[diesel(table_name = crate::database::schema::object)]
pub struct NewModel {
    pub source: String,
    pub type_: String,
    pub location: String,
    pub data: String,
}
impl NewModel {
    pub async fn create(
        &self,
        ctx: &crate::database::graphql::Context,
    ) -> juniper::FieldResult<Model> {
        use diesel::RunQueryDsl;
        let mut client = ctx.pool.get()?;
        Ok(
            diesel::insert_into(crate::database::schema::object::dsl::object)
                .values(self)
                .get_result(&mut client)?,
        )
    }
}
