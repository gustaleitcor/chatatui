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
        participant_id -> Int4,
        date -> Timestamp,
    }
}

diesel::table! {
    participants (id) {
        id -> Int4,
        chat_id -> Int4,
        user_id -> Int4,
        is_admin -> Bool,
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

diesel::joinable!(messages -> participants (participant_id));
diesel::joinable!(participants -> chats (chat_id));
diesel::joinable!(participants -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(chats, messages, participants, users,);
