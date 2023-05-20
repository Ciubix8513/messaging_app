use iced::{
    widget::{button, column, container, row, text, text_input},
    Alignment, Color, Length,
};

use crate::main_window::{MainForm, Message};

impl MainForm {
    pub fn login_view<'a>(&self) -> iced::Element<'a, Message> {
        let login_text = text("Login").size(40);
        let width = 200;

        let login = text_input("Login..", &self.login_data.login_textbox)
            .width(width)
            .on_input(Message::LoginChanged);
        let password = text_input("Password..", &self.login_data.password_textbox)
            .password()
            .width(width)
            .on_input(Message::PasswordChanged);
        let error = text(&self.login_data.error_message).style(Color::from_rgb(1.0, 0.0, 0.0));
        let signup_button = button("Signup").on_press(Message::LoginViewSignupButtonPressed);
        let login_button = button("Log in").on_press(Message::LoginButtonPressed);

        let mut content = column![login_text, login, password];
        if self.login_data.show_error_message {
            content = content.push(error);
        }
        content = content
            .push(row![login_button, signup_button].spacing(5))
            .spacing(15)
            .align_items(Alignment::Center);
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}
