// @generated automatically by Diesel CLI.

diesel::table! {
    group_chat_members (chat_id, user_id) {
        chat_id -> Integer,
        user_id -> Integer,
    }
}

diesel::table! {
    group_chats (chat_id) {
        chat_id -> Integer,
        chat_name -> Varchar,
        created_at -> Datetime,
    }
}

diesel::table! {
    messages (message_id) {
        message_id -> Integer,
        chat_id -> Integer,
        user_id -> Integer,
        message_text -> Text,
        sent_at -> Datetime,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(group_chat_members -> group_chats (chat_id));
diesel::joinable!(group_chat_members -> users (user_id));
diesel::joinable!(messages -> group_chats (chat_id));
diesel::joinable!(messages -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    group_chat_members,
    group_chats,
    messages,
    users,
);
