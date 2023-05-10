use iced::{
    widget::{button, column, container, row, text},
    Length,
};
use reqwest::Method;

use crate::{
    grimoire,
    main_window::{MainForm, Message, WindowMode},
    window_structs::LoginData,
    CLIENT, COOKIE_STORE,
};

impl MainForm {
    pub fn update_chat_list(&mut self) {}

    pub fn logout(&mut self) {
        let c = COOKIE_STORE
            .lock()
            .unwrap()
            .iter_any()
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .value()
            .to_string();
        println!("SENDING COOKE, {}", c);
        let result = CLIENT
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .request(Method::DELETE, grimoire::AUTH_LOGOUT.clone())
            //This is so sketch lol
            .header("cookie", format!("id={}", c))
            .send();
        let result = result.unwrap();
        if !result.status().is_success() {
            println!("Logout error {}", result.text().unwrap());
        }

        self.winodow_mode = WindowMode::Login;
        self.login_data = LoginData::default();
    }

    pub fn messaging_view<'a>(&self) -> iced::Element<'a, Message> {
        let top_bar = {
            //Start with the top bar
            let logout = button("Logout").on_press(Message::LogoutButtonPressed);
            container(row![logout])
                .align_x(iced::alignment::Horizontal::Right)
                .height(35)
                .padding(2)
                .width(Length::Fill)
        };
        let side_bar = {
            let contents = text("");
            container(row![contents])
                .align_x(iced::alignment::Horizontal::Right)
                .height(35)
                .padding(2)
                .width(Length::Fill)
        };
        container(column![top_bar, side_bar]).into()
    }
}
