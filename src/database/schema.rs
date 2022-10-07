diesel::table! {
    events (id) {
        id -> Int4,
        template -> Varchar,
        // type_ -> Etype,
        #[sql_name = "type"]
        type_ -> Varchar,
        source -> Text,
        data -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {

    keywords (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
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
