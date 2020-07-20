table! {
    languages (id) {
        id -> Integer,
        nome -> Varchar,
    }
}

table! {
    translations (id) {
        id -> Integer,
        language_from -> Integer,
        language_to -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        nome -> Varchar,
        login -> Varchar,
        psswd -> Varchar,
        date_inserted -> Nullable<Datetime>,
        date_updated -> Nullable<Datetime>,
    }
}

table! {
    words (id) {
        id -> Integer,
        nome -> Varchar,
        id_language -> Integer,
    }
}

joinable!(words -> languages (id_language));

allow_tables_to_appear_in_same_query!(
    languages,
    translations,
    users,
    words,
);
