// @generated automatically by Diesel CLI.

diesel::table! {
    chats (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        is_public -> Bool,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        content -> Text,
        chat_id -> Int4,
        user_id -> Nullable<Int4>,
        date -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(messages -> chats (chat_id));
diesel::joinable!(messages -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    chats,
    messages,
    users,
);
