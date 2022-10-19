diesel::table! {
    secrets (id) {
        id -> Integer,
        name -> Varchar,
        value -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    events (id) {
        id -> Integer,
        template -> Varchar,
        #[sql_name = "type"]
        type_ -> Integer,
        source -> Text,
        data -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    keywords (id) {
        id -> Integer,
        #[sql_name = "type"]
        type_ -> Integer,
        value -> Varchar,
        created_at -> Timestamp,
        last_consulted -> Timestamp,
    }
}

diesel::table! {
    events_keywords (event, keyword) {
        event -> Int4,
        keyword -> Int4,
    }
}

diesel::joinable!(events_keywords -> events (event));
diesel::joinable!(events_keywords -> keywords (keyword));

diesel::allow_tables_to_appear_in_same_query!(events, keywords, events_keywords,);
