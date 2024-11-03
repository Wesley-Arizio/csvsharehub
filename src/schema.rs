// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Text,
        original_name -> Text,
        file_path -> Text,
        content_type -> Text,
        size -> BigInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
