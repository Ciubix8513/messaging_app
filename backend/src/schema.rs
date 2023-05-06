// @generated automatically by Diesel CLI.

diesel::table! {
    Group_Chat_Members (chat_id, user_id) {
        chat_id -> Integer,
        user_id -> Integer,
    }
}

diesel::table! {
    Group_Chats (chat_id) {
        chat_id -> Integer,
        chat_name -> Varchar,
        created_at -> Datetime,
    }
}

diesel::table! {
    Messages (message_id) {
        message_id -> Integer,
        chat_id -> Integer,
        user_id -> Integer,
        message_text -> Text,
        sent_at -> Datetime,
    }
}

diesel::table! {
    Users (user_id) {
        user_id -> Integer,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(Group_Chat_Members -> Group_Chats (chat_id));
diesel::joinable!(Group_Chat_Members -> Users (user_id));
diesel::joinable!(Messages -> Group_Chats (chat_id));
diesel::joinable!(Messages -> Users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    Group_Chat_Members,
    Group_Chats,
    Messages,
    Users,
);
