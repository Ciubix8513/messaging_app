use common_structs::Login;
use iced::{
    widget::{button, column, container, text, text_input},
    Alignment, Color, Length,
};
use reqwest::Method;

use super::main_window::*;
use crate::{grimoire, CLIENT, COOKIE_STORE};

impl MainForm {
    pub fn login(&mut self) {
        let body = Login {
            username: self.login_data.login_textbox.clone(),
            password: self.login_data.password_textbox.clone(),
        };
        let response = CLIENT
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .request(Method::POST, grimoire::AUTH_LOGIN.clone())
            .json(&body)
            .send()
            .unwrap();
        println!("{:#?}", response);
        println!("Num cookies {}", response.cookies().count());

        if !response.status().is_success() {
            self.login_data.error_message = String::from("Wrong username or password");
            self.login_data.show_error_message = true;
            self.login_data.password_textbox.clear();
            return;
        }

        let cnt = COOKIE_STORE.lock().unwrap().iter_any().count();
        println!("{} cookies in the store", cnt)

        // COOKIE_STORE.lock().unwrap().insert_raw(
        //     response.cookies().collect::<Vec<_>>().first().unwrap() as cookie::Cookie,
        //     response.url(),
        // );

        // {
        //     self.login_data.show_error_message = false;
        //     println!("Logged in successfully");
        //     self.winodow_mode = WindowMode::Main;
        //     return;
        //     //Open the other window
        // }
    }
    pub fn login_view<'a>(&self) -> iced::Element<'a, Message> {
        let width = 200;
        let login = text_input("Login..", &self.login_data.login_textbox)
            .width(width)
            .on_input(Message::LoginChanged);
        let password = text_input("Password..", &self.login_data.password_textbox)
            .password()
            .width(width)
            .on_input(Message::PasswordChanged);
        let error = text(&self.login_data.error_message).style(Color::from_rgb(1.0, 0.0, 0.0));
        let login_button = button("Log in").on_press(Message::ButtonPressed);

        let mut content = column![login, password];
        if self.login_data.show_error_message {
            content = content.push(error);
        }
        content = content
            .push(login_button)
            .spacing(30)
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
