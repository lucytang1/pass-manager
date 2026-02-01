// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        user_key -> Text,
        salt -> Text,
        vault -> Text,
    }
}
