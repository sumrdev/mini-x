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
        #[max_length = 255]
        text -> Varchar,
        pub_date -> Nullable<Timestamp>,
        flagged -> Nullable<Int4>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 100]
        pw_hash -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    followers,
    messages,
    users,
);
