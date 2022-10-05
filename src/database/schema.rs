diesel::table! {
    event (id) {
        id -> Int4,
        template -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        source -> Text,
        data -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    keyword (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
        value -> Varchar,
        created_at -> Timestamp,
        last_consulted -> Timestamp,
    }
}