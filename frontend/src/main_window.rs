use iced::Sandbox;

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
pub enum WindowMode {
    #[default]
    Login,
    SignUp,
}
#[derive(Default)]
pub struct MainForm {
    pub login_data: LoginData,
    pub winodow_mode: WindowMode,
    pub signup_data: SignupData,
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
            Message::LoginViewSignupButtonPressed => self.winodow_mode = WindowMode::SignUp,
            Message::BackButtonPressed => self.winodow_mode = WindowMode::Login,
            //Signup stuff
            Message::UsernameChanged(v) => self.signup_data.username_textbox = v,
            Message::EmailChanged(v) => self.signup_data.email_textbox = v,
            Message::SignupPasswordChanged(v, i) => self.signup_data.password_textbox[i] = v,
            Message::SignupButtonPressed => self.signup(),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        match self.winodow_mode {
            WindowMode::Login => self.login_view(),
            WindowMode::SignUp => self.signup_view(),
        }
    }

    fn new() -> Self {
        MainForm::default()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
