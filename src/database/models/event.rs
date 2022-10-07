use diesel::ExpressionMethods;

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
    pub template: String,
    /// Event type of the event
    pub type_: Type,
    /// The website or server where the data was stored
    pub source: String,
    /// The found data
    pub data: String,
    /// The date when the event was created
    pub created_at: chrono::NaiveDateTime,
}
impl Model {
    pub async fn read(ctx: &crate::database::graphql::Context) -> juniper::FieldResult<Vec<Self>> {
        // access the database
        use diesel::RunQueryDsl;
        let mut client = ctx.pool.get()?;
        Ok(crate::database::schema::event::dsl::event.load::<Self>(&mut client)?)
    }

    pub async fn filter_by_type(ctx: &crate::database::graphql::Context, type_: Type) -> juniper::FieldResult<Vec<Self>> {
        // access the database
        use diesel::{QueryDsl,RunQueryDsl};
        let mut client = ctx.pool.get()?;
        Ok(crate::database::schema::event::table.filter(crate::database::schema::event::columns::type_.eq(type_)).load::<Self>(&mut client)?)
    }

    pub async fn delete(
        ctx: &crate::database::graphql::Context,
        id: i32,
    ) -> juniper::FieldResult<Model> {
        use diesel::{QueryDsl, RunQueryDsl};
        let mut client = ctx.pool.get()?;
        Ok(diesel::delete(
            crate::database::schema::event::dsl::event
                .filter(crate::database::schema::event::dsl::id.eq(id)),
        )
        .get_result(&mut client)?)
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(Clone,Debug,Default,Eq,PartialEq,serde::Deserialize,juniper::GraphQLEnum,diesel::AsExpression,diesel::FromSqlRow,diesel::QueryId)]
#[graphql(name = "EventType")]
#[diesel(sql_type = crate::database::schema::sql_types::Etype)]
pub enum Type{
    #[default]
    Paste,
}
impl diesel::serialize::ToSql<crate::database::schema::sql_types::Etype, diesel::pg::Pg> for Type {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
        use std::io::Write;
        match *self {
            Self::Paste => out.write_all(b"paste")?,
        }
        Ok(diesel::serialize::IsNull::No)
    }
}
impl diesel::deserialize::FromSql<crate::database::schema::sql_types::Etype, diesel::pg::Pg> for Type {
    fn from_sql(bytes: diesel::pg::PgValue) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"paste" => Ok(Self::Paste),
            _ => Err("unrecognized enum variant".into()),
        }
    }
}
impl std::str::FromStr for Type {
    type Err = crate::error::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "paste" => Ok(Self::Paste),
            err => Err(Self::Err::GenericError(format!("unrecognized type {}",err))),
        }
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[derive(
    Clone, Debug, PartialEq, serde::Deserialize, juniper::GraphQLInputObject, diesel::Insertable,
)]
#[graphql(name = "NewEvent")]
#[diesel(table_name = crate::database::schema::event)]
pub struct NewModel {
    pub template: String,
    pub type_: Type,
    pub source: String,
    pub data: String,
}
impl NewModel {
    pub async fn save(
        &self,
        pool: &crate::database::Connection,
    ) -> std::result::Result<Model, Box<dyn std::error::Error>> {
        // save it into the database
        use diesel::RunQueryDsl;
        let mut client = pool.get()?;
        let model: Model = diesel::insert_into(crate::database::schema::event::dsl::event)
            .values(self)
            .get_result(&mut client)?;
        Ok(model)
    }

    pub async fn create(
        &self,
        ctx: &crate::database::graphql::Context,
    ) -> juniper::FieldResult<Model> {
        Ok(self.save(&ctx.pool).await?)
    }
}
