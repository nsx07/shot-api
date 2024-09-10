// @generated automatically by Diesel CLI.

diesel::table! {
    shot (id) {
        id -> Integer,
        #[max_length = 255]
        lat -> Varchar,
        #[max_length = 255]
        lon -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        image64 -> Text,
        #[max_length = 255]
        location -> Varchar,
        #[max_length = 255]
        hashtags -> Varchar,
        description -> Text,
        created_at -> Nullable<Timestamp>,
    }
}
