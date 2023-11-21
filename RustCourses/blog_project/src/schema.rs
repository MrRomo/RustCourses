// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        slug -> Varchar,
        body -> Text,
    }
}
