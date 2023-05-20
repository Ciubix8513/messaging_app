use iced::{
    widget::{button, column, container, text, text_input},
    Alignment, Color, Length,
};

use crate::main_window::{MainForm, Message};

impl MainForm {
    pub fn signup_view<'a>(&self) -> iced::Element<'a, Message> {
        let signup_text = text("Signup  ").size(40);
        let width = 200;

        let back_button = button("<").on_press(Message::BackButtonPressed);
        let username = text_input("Username..", &self.signup_data.username_textbox)
            .width(width)
            .on_input(Message::UsernameChanged);
        let email = text_input("Email..", &self.signup_data.email_textbox)
            .width(width)
            .on_input(Message::EmailChanged);
        let password = text_input("Password..", &self.signup_data.password_textbox[0])
            .password()
            .width(width)
            .on_input(|x| Message::SignupPasswordChanged(x, 0));
        let password1 = text_input("Repeat password..", &self.signup_data.password_textbox[1])
            .password()
            .width(width)
            .on_input(|x| Message::SignupPasswordChanged(x, 1));
        let error = text(&self.signup_data.error_message).style(Color::from_rgb(1.0, 0.0, 0.0));
        let login_button = button("Sign up").on_press(Message::SignupButtonPressed);

        let mut content = column![signup_text, username, email, password, password1];
        if self.signup_data.show_error_message {
            content = content.push(error);
        }
        content = content
            .push(login_button)
            .spacing(20)
            .align_items(Alignment::Center);
        let top = container(back_button)
            .width(Length::Fill)
            .padding(5)
            .align_x(iced::alignment::Horizontal::Left);
        let bottom = container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10);
        // .center_x()
        // .center_y();
        container(container(column![top, bottom]).width(200).height(500))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into()
    }
}
