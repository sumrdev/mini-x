// @generated automatically by Diesel CLI.

diesel::table! {
    followers (who_id, whom_id) {
        who_id -> Int4,
        whom_id -> Int4,
    }
}

diesel::table! {
    messages (message_id) {
        message_id -> Int4,
        author_id -> Int4,
        text -> Text,
        pub_date -> Text,
        flagged -> Int4,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        username -> Text,
        email -> Text,
        pw_hash -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(followers, messages, users,);
