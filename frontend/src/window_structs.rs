use common_lib::{GetInvites, GetMessage};

use crate::main_window::Message;

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
pub struct TextInputModalData {
    pub title: String,
    pub placeholder: String,
    pub show_modal: bool,
    pub modal_text: String,
    pub message: Message,
}

#[derive(Default)]
pub enum MessageViewMode {
    #[default]
    Messages,
    Invites,
}

#[derive(Default)]
pub struct MessagingData {
    pub chats: Vec<Chat>,
    pub selected_chat: Option<i32>,
    pub textinput_modal_data: TextInputModalData,
    pub show_error_modal: bool,
    pub error_message: String,
    pub messages: Vec<GetMessage>,
    pub invites: Vec<GetInvites>,
    pub mode: MessageViewMode,
    pub current_message: String,
}
