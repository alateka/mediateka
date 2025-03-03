// @generated automatically by Diesel CLI.

diesel::table! {
    image (id) {
        id -> Integer,
        title -> Text,
        path -> Text,
    }
}

diesel::table! {
    music (id) {
        id -> Integer,
        title -> Text,
        path -> Text,
        artist -> Text,
    }
}

diesel::table! {
    video (id) {
        id -> Integer,
        title -> Text,
        path -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    image,
    music,
    video,
);
