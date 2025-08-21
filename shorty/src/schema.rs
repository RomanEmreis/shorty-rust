// @generated automatically by Diesel CLI.

diesel::table! {
    shorty_urls (token) {
        token -> Text,
        url -> Text,
        created_at -> Timestamp,
    }
}
