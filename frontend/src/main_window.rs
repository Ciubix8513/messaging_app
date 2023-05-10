use iced::Sandbox;

#[derive(Default)]
pub struct LoginData {
    pub login_textbox: String,
    pub password_textbox: String,
    pub error_message: String,
    pub show_error_message: bool,
}

#[derive(Default)]

pub enum WindowMode {
    #[default]
    Login,
    Main,
}
#[derive(Default)]
pub struct MainForm {
    pub login_data: LoginData,
    pub winodow_mode: WindowMode,
}

#[derive(Debug, Clone)]
pub enum Message {
    LoginChanged(String),
    PasswordChanged(String),
    ButtonPressed,
}

impl Sandbox for MainForm {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Login")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::LoginChanged(l) => self.login_data.login_textbox = l,
            Message::PasswordChanged(l) => self.login_data.password_textbox = l,
            Message::ButtonPressed => self.login(),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        match self.winodow_mode {
            WindowMode::Login => self.login_view(),
            WindowMode::Main => todo!(),
        }
    }

    fn new() -> Self {
        MainForm::default()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
