// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        user_id -> Integer,
        stored_on_server -> Bool,
        LOCATION -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        username -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
