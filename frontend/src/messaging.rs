use common_structs::GetChat;
use iced::{
    alignment::{self, Horizontal},
    widget::{button, column, container, row, text, text_input},
    Length,
};
use iced_aw::{Card, Modal};
use reqwest::Method;

use crate::{
    grimoire,
    main_window::{MainForm, Message, WindowMode},
    window_structs::{Chat, LoginData},
    CLIENT, COOKIE_STORE,
};

impl MainForm {
    pub fn update_chat_list(&mut self) {
        let result = CLIENT
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .request(Method::GET, grimoire::CHATS_GET.clone())
            .header(
                "cookie",
                format!(
                    "id={}",
                    COOKIE_STORE
                        .lock()
                        .unwrap()
                        .iter_unexpired()
                        .collect::<Vec<_>>()
                        .first()
                        .unwrap()
                        .value()
                ),
            )
            .send()
            .unwrap();
        if !result.status().is_success() {
            self.error_message(result.text().unwrap());
            return;
        }
        let data: Vec<GetChat> = result.json().unwrap();
        self.messaging_data.chats = data
            .iter()
            .map(|i| Chat {
                chat_id: i.chat_id,
                chat_name: i.chat_name.clone(),
            })
            .collect();
    }

    pub fn error_message(&mut self, message: String) {
        println!("ERROR {}", message)
    }

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

    pub fn messaging_view(&self) -> iced::Element<'_, Message> {
        let top_bar = {
            //Start with the top bar
            let logout = button("Logout").on_press(Message::LogoutButtonPressed);
            container(row![logout])
                .align_x(alignment::Horizontal::Right)
                .height(35)
                .padding(2)
                .width(Length::Fill)
        };
        let side_bar = {
            let contents = text("").height(Length::Fill);
            let bottom_things = {
                let new_chat = button(
                    text("New chat")
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .width(Length::Fill),
                )
                .on_press(Message::CreateChatButtonPressed)
                .width(Length::Fill);
                let invite = button(
                    text("Invite")
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .width(Length::Fill),
                )
                .width(Length::Fill);

                container(column![new_chat, invite].spacing(5))
                    .align_y(alignment::Vertical::Bottom)
                    .height(100)
            };

            container(column![contents, bottom_things])
                .align_x(alignment::Horizontal::Left)
                .height(Length::Fill)
                .width(200)
                .padding(2)
        };

        let main_content = container(column![top_bar, side_bar]);
        Modal::new(
            self.messaging_data.show_create_chat_modal,
            main_content,
            || {
                Card::new(
                    text("Create chat"),
                    text_input("Chat name", &self.messaging_data.create_chat_text.clone())
                        .on_input(Message::CreateChatModalTextChange),
                )
                .foot(
                    row![
                        button(text("Cancel").horizontal_alignment(Horizontal::Center))
                            .width(Length::Fill)
                            .on_press(Message::CloseCreateChatModal),
                        button(text("Ok").horizontal_alignment(Horizontal::Center))
                            .width(Length::Fill)
                            .on_press(Message::ConfirmCreateChat),
                    ]
                    .spacing(5),
                )
                .max_width(250.0)
                .on_close(Message::CloseCreateChatModal)
                .into()
            },
        )
        .backdrop(Message::CloseCreateChatModal)
        .on_esc(Message::CloseCreateChatModal)
        .into()
    }
}
