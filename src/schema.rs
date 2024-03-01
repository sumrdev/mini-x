// @generated automatically by Diesel CLI.

diesel::table! {
    user (user_id) {
        user_id -> Integer,
        username -> Varchar,
        email -> Text,
        pw_hash -> Varchar,
    }
}

diesel::table! {
    follower (who_id, whom_id) {
        who_id -> Int4,
        whom_id -> Int4,
    }
}

diesel::table! {
    message (message_id) {
        message_id -> Int4,
        author_id -> Int4,
        text -> Text,
        pub_date -> Int4,
        flagged -> Int4,
    }
}