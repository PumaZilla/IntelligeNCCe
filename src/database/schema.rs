pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "etype"))]
    pub struct Etype;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ktype"))]
    pub struct Ktype;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Etype;

    event (id) {
        id -> Int4,
        template -> Varchar,
        #[sql_name = "type"]
        type_ -> Etype,
        source -> Text,
        data -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Ktype;

    keyword (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Ktype,
        value -> Varchar,
        created_at -> Timestamp,
        last_consulted -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    event,
    keyword,
);