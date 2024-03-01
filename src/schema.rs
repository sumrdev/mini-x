// @generated automatically by Diesel CLI.

diesel::table! {
    follower (who_id, whom_id) {
        who_id -> Integer,
        whom_id -> Integer,
    }
}

diesel::table! {
    message (message_id) {
        message_id -> Integer,
        author_id -> Integer,
        text -> Text,
        pub_date -> Integer,
        flagged -> Integer,
    }
}

diesel::table! {
    user (user_id) {
        user_id -> Integer,
        username -> Text,
        email -> Text,
        pw_hash -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    follower,
    message,
    user,
);
