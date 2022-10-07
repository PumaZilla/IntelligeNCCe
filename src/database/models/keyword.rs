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
    pub type_: Type,
    /// The keyword
    pub value: String,
    /// The date when the keyword was created
    pub created_at: chrono::NaiveDateTime,
    /// The date when the keyword was last consulted
    pub last_consulted: chrono::NaiveDateTime,
}
impl Model {
    pub async fn all(pool: &crate::database::DBConnection) -> crate::error::Result<Vec<Model>> {
        use diesel::RunQueryDsl;
        let mut client = pool
            .get()
            .map_err(|e| crate::error::Error::DatabasePoolError(e.to_string()))?;
        Ok(crate::database::schema::keyword::table
            .load::<Model>(&mut client)
            .map_err(|e| crate::error::Error::DatabaseReadError(e.to_string()))?)
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
    Clone,
    Debug,
    Default,
    PartialEq,
    serde::Deserialize,
    juniper::GraphQLEnum,
    diesel::AsExpression,
    diesel::FromSqlRow,
)]
#[graphql(name = "KeywordType")]
#[diesel(sql_type = crate::database::schema::sql_types::Ktype)]
pub enum Type {
    #[default]
    Text,
    Credential,
    Domain,
    Email,
    IP,
    Phone,
    URL,
    Username,
}
impl diesel::serialize::ToSql<crate::database::schema::sql_types::Ktype, diesel::pg::Pg> for Type {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        use std::io::Write;
        match *self {
            Self::Text => out.write_all(b"text")?,
            Self::Credential => out.write_all(b"credential")?,
            Self::Domain => out.write_all(b"domain")?,
            Self::Email => out.write_all(b"email")?,
            Self::IP => out.write_all(b"ip")?,
            Self::Phone => out.write_all(b"phone")?,
            Self::URL => out.write_all(b"url")?,
            Self::Username => out.write_all(b"username")?,
        }
        Ok(diesel::serialize::IsNull::No)
    }
}
impl diesel::deserialize::FromSql<crate::database::schema::sql_types::Ktype, diesel::pg::Pg>
    for Type
{
    fn from_sql(bytes: diesel::pg::PgValue) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"text" => Ok(Self::Text),
            b"credential" => Ok(Self::Credential),
            b"domain" => Ok(Self::Domain),
            b"email" => Ok(Self::Email),
            b"ip" => Ok(Self::IP),
            b"phone" => Ok(Self::Phone),
            b"url" => Ok(Self::URL),
            b"Username" => Ok(Self::Username),
            _ => Err("unrecognized enum variant".into()),
        }
    }
}
impl std::str::FromStr for Type {
    type Err = crate::error::Error;
    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "text" => Ok(Self::Text),
            "credential" => Ok(Self::Credential),
            "domain" => Ok(Self::Domain),
            "email" => Ok(Self::Email),
            "ip" => Ok(Self::IP),
            "phone" => Ok(Self::Phone),
            "url" => Ok(Self::URL),
            "Username" => Ok(Self::Username),
            err => Err(Self::Err::GenericError(format!(
                "unrecognized type {}",
                err
            ))),
        }
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
    pub type_: Type,
    /// The keyword
    pub value: String,
}
impl NewModel {
    pub async fn save(
        &self,
        pool: &crate::database::DBConnection,
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
