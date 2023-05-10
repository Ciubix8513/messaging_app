use crate::window_structs::*;
use iced::Sandbox;

#[derive(Default)]
pub enum WindowMode {
    #[default]
    Login,
    SignUp,
    Messaging,
}
#[derive(Default)]
pub struct MainForm {
    pub login_data: LoginData,
    pub winodow_mode: WindowMode,
    pub signup_data: SignupData,
    pub messaging_data: MessagingData,
}

#[derive(Debug, Clone)]
pub enum Message {
    //Login
    LoginChanged(String),
    PasswordChanged(String),
    LoginButtonPressed,
    LoginViewSignupButtonPressed,
    //Signup
    UsernameChanged(String),
    EmailChanged(String),
    SignupPasswordChanged(String, usize),
    BackButtonPressed,
    SignupButtonPressed,
    //Messaging
    LogoutButtonPressed,
    CreateChatButtonPressed,
    CloseCreateChatModal,
    ConfirmCreateChat,
    CreateChatModalTextChange(String),
}

impl Sandbox for MainForm {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Login")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            //Login stuff
            Message::LoginChanged(v) => self.login_data.login_textbox = v,
            Message::PasswordChanged(v) => self.login_data.password_textbox = v,
            Message::LoginButtonPressed => self.login(),
            Message::LoginViewSignupButtonPressed => {
                //Clear signup data
                self.signup_data = SignupData::default();
                self.winodow_mode = WindowMode::SignUp
            }
            Message::BackButtonPressed => self.winodow_mode = WindowMode::Login,
            //Signup stuff
            Message::UsernameChanged(v) => self.signup_data.username_textbox = v,
            Message::EmailChanged(v) => self.signup_data.email_textbox = v,
            Message::SignupPasswordChanged(v, i) => self.signup_data.password_textbox[i] = v,
            Message::SignupButtonPressed => self.signup(),
            Message::LogoutButtonPressed => self.logout(),
            Message::CreateChatButtonPressed => {
                self.messaging_data.create_chat_text.clear();
                self.messaging_data.show_create_chat_modal = true
            }
            Message::CloseCreateChatModal => self.messaging_data.show_create_chat_modal = false,
            Message::ConfirmCreateChat => self.messaging_data.show_create_chat_modal = false,
            Message::CreateChatModalTextChange(v) => self.messaging_data.create_chat_text = v,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        match self.winodow_mode {
            WindowMode::Login => self.login_view(),
            WindowMode::SignUp => self.signup_view(),
            WindowMode::Messaging => self.messaging_view(),
        }
    }

    fn new() -> Self {
        MainForm::default()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
