table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
    }
}

table! {
    messages (id) {
        id -> Int4,
        user_id -> Int4,
        content -> Text,
        timestamp -> Timestamp,
    }
}
