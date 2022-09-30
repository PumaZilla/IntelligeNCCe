diesel::table! {
    object (id) {
        id -> Int4,
        source -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        location -> Text,
        data -> Text,
    }
}
