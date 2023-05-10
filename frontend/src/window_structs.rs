#[derive(Default)]
pub struct LoginData {
    pub login_textbox: String,
    pub password_textbox: String,
    pub error_message: String,
    pub show_error_message: bool,
}

#[derive(Default)]
pub struct SignupData {
    pub username_textbox: String,
    pub email_textbox: String,
    pub password_textbox: [String; 2],
    pub error_message: String,
    pub show_error_message: bool,
}

#[derive(Default)]
pub struct Chat {
    pub chat_id: i32,
    pub chat_name: String,
}

#[derive(Default)]
pub struct MessagingData {
    pub chats: Vec<Chat>,
    pub selected_chat: Option<i32>,
    pub show_create_chat_modal: bool,
    pub create_chat_text: String,
    pub show_error_modal: bool,
    pub error_message: String,
}
