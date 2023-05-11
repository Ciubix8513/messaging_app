use common_structs::{GetChat, GetMessage, SendInvite, SendMessage};
use iced::{
    alignment::{self, Horizontal},
    theme::Container,
    widget::{button, column, container, row, scrollable, text, text_input, Column},
    Color, Length,
};
use iced_aw::{Card, Modal};
use reqwest::{Method, StatusCode};

use crate::{
    grimoire,
    main_window::{MainForm, Message, WindowMode, SCROLLABLE_ID},
    window_structs::{Chat, LoginData},
    CLIENT, COOKIE_STORE,
};

impl MainForm {
    pub fn send_message(&mut self) {
        let body = SendMessage {
            chat_id: self.messaging_data.selected_chat.unwrap(),
            text: self.messaging_data.current_message.clone(),
        };
        let result = CLIENT
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .post(grimoire::MESSAGES_SEND.clone())
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
            .json(&body)
            .send()
            .unwrap();
        if !result.status().is_success() {
            let status = result.status();
            self.error_message(result.text().unwrap(), status);
            return;
        }
        self.messaging_data.current_message.clear();
        self.load_messages();
    }

    pub fn accept_invite(&mut self, id: i32) {
        let result = CLIENT
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .post(grimoire::INVITES_ACCEPT.clone())
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
            .json(&id)
            .send()
            .unwrap();
        if !result.status().is_success() {
            let status = result.status();
            self.error_message(result.text().unwrap(), status);
            return;
        }
        self.messaging_data.invites.retain(|i| i.invite_id != id);
    }

    pub fn decline_invite(&mut self, id: i32) {
        let result = CLIENT
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .post(grimoire::INVITES_REJECT.clone())
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
            .json(&id)
            .send()
            .unwrap();
        if !result.status().is_success() {
            let status = result.status();
            self.error_message(result.text().unwrap(), status);
            return;
        }
        self.messaging_data.invites.retain(|i| i.invite_id != id);
    }

    pub fn send_invite(&mut self) {
        let body = SendInvite {
            chat_id: self.messaging_data.selected_chat.unwrap(),
            recipient_name: self.messaging_data.textinput_modal_data.modal_text.clone(),
        };
        let result = CLIENT
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .post(grimoire::INVITES_SEND.clone())
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
            .json(&body)
            .send()
            .unwrap();
        if !result.status().is_success() {
            let status = result.status();
            self.error_message(result.text().unwrap(), status);
            return;
        }
    }

    pub fn update_invites_list(&mut self) {
        let result = CLIENT
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .get(grimoire::INVITES_GET.clone())
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

        if !result.status().is_success() && result.status() != reqwest::StatusCode::NOT_FOUND {
            let status = result.status();
            self.error_message(result.text().unwrap(), status);
            return;
        }
        self.messaging_data.invites = result.json().unwrap_or(Vec::default());
    }

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
        if !result.status().is_success() && result.status() != reqwest::StatusCode::NOT_FOUND {
            let status = result.status();
            self.error_message(result.text().unwrap(), status);
            return;
        }
        let data: Vec<GetChat> = result.json().unwrap_or(Vec::default());
        self.messaging_data.chats = data
            .iter()
            .map(|i| Chat {
                chat_id: i.chat_id,
                chat_name: i.chat_name.clone(),
            })
            .collect();
    }

    pub fn error_message(&mut self, message: String, code: StatusCode) {
        println!("ERROR {:#?} / {}", code, message);
        self.messaging_data.show_error_modal = true;
        self.messaging_data.error_message = message;
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

    pub fn create_chat(&mut self) {
        let result = CLIENT
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .request(Method::POST, grimoire::CHATS_CREATE.clone())
            .json(&self.messaging_data.textinput_modal_data.modal_text)
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
            let status = result.status();
            self.error_message(result.text().unwrap(), status);
            return;
        }
        self.update_chat_list();
    }

    pub fn load_messages(&mut self) -> bool {
        if self.messaging_data.selected_chat.is_none() {
            return false;
        }
        let result = CLIENT
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .get(grimoire::MESSAGES_GET.clone())
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
            .query(&[("id", self.messaging_data.selected_chat)])
            .send()
            .unwrap();
        if !result.status().is_success() {
            let status = result.status();
            self.error_message(result.text().unwrap(), status);
            return false;
        }
        let new = result.json::<Vec<GetMessage>>().unwrap();
        if new.len() == self.messaging_data.messages.len() {
            return false;
        }
        self.messaging_data.messages = new;
        true
    }

    pub fn messaging_view(&self) -> iced::Element<'_, Message> {
        let top_bar = {
            //Start with the top bar
            let logout = button("Logout").on_press(Message::LogoutButtonPressed);
            let invites = button("Invites").on_press(Message::InvitesButtonPressed);

            container(row![invites, logout].spacing(2))
                .align_x(alignment::Horizontal::Right)
                .height(35)
                .padding(2)
                .width(Length::Fill)
                .style(Container::Box)
        };
        let side_bar = {
            // let contents = text("").height(Length::Fill);
            let contents = self
                .messaging_data
                .chats
                .iter()
                .map(|t| {
                    button(text(format!("{}", t.chat_name)))
                        .on_press(Message::SelectChat(t.chat_id))
                })
                .fold(Column::new(), |acc, x| acc.push(x))
                .spacing(5);
            //fold(column,|t| text(format!("{}",t.name)))
            let bottom_things = {
                let new_chat = button(
                    text("New chat")
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .width(Length::Fill),
                )
                .on_press(Message::CreateChatButtonPressed)
                .width(Length::Fill);
                let mut invite = button(
                    text("Invite")
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .width(Length::Fill),
                )
                .width(Length::Fill);

                if self.messaging_data.selected_chat.is_some() {
                    invite = invite.on_press(Message::InviteButtonPressed);
                }

                container(column![new_chat, invite].spacing(5))
                    .align_y(alignment::Vertical::Bottom)
                    .height(100)
            };

            container(column![
                text("Chats:"),
                scrollable(contents).height(Length::Fill),
                bottom_things
            ])
            .align_x(alignment::Horizontal::Left)
            .height(Length::Fill)
            .width(200)
            .padding(2)
            .style(Container::Box)
        };
        let date_color = Color::from_rgb(0.6, 0.6, 0.6);
        let main_view = match self.messaging_data.mode {
            crate::window_structs::MessageViewMode::Messages => {
                if self.messaging_data.selected_chat.is_some() {
                    container(column![
                        scrollable(
                            self.messaging_data
                                .messages
                                .iter()
                                .map(|i| {
                                    let uname = text(&i.username);
                                    let date = text(i.sent_at).style(date_color.clone());
                                    let body = text(&i.message_text);
                                    container(column![row![uname, date].spacing(5), body])
                                })
                                .fold(Column::new(), |c, i| c.push(i)),
                        )
                        .height(Length::Fill)
                        .id(SCROLLABLE_ID.clone()),
                        {
                            let input = text_input("...", &self.messaging_data.current_message)
                                .on_input(Message::MessageEdited);
                            let mut send_button = button("Send");
                            if !self.messaging_data.current_message.is_empty() {
                                send_button = send_button.on_press(Message::SendMessage);
                            }
                            row![input, send_button].spacing(5)
                        }
                    ])
                } else {
                    //Kinda hacky but it's fine
                    container(text(""))
                }
            }
            crate::window_structs::MessageViewMode::Invites => container(
                column![
                    text("Invites:"),
                    if !self.messaging_data.invites.is_empty() {
                        scrollable(
                            self.messaging_data
                                .invites
                                .iter()
                                .map(|i| {
                                    container({
                                        let c_name = text(i.chat_name.clone());
                                        let s_name =
                                            text(format!("from: {}", i.sender_name.clone()));
                                        let date = text(i.created_at).style(date_color.clone());
                                        let accept_btn = button("Accpet")
                                            .on_press(Message::AcceptInvite(i.invite_id));
                                        let decline_btn = button("Decline")
                                            .on_press(Message::DeclineInvite(i.invite_id));
                                        column![
                                            row![c_name, date].spacing(5),
                                            s_name,
                                            row![accept_btn, decline_btn].spacing(3)
                                        ]
                                        .spacing(5)
                                    })
                                    .padding(5)
                                })
                                .fold(Column::new(), |c, i| c.push(i)),
                        )
                    } else {
                        scrollable(text("None"))
                    }
                ]
                .spacing(10),
            ),
        }
        .padding(10);

        let main_content = container(column![top_bar, row![side_bar, main_view]]);
        let text_input_modal = Modal::new(
            self.messaging_data.textinput_modal_data.show_modal,
            main_content,
            || {
                Card::new(
                    text(&self.messaging_data.textinput_modal_data.title),
                    text_input(
                        &self.messaging_data.textinput_modal_data.placeholder,
                        &self.messaging_data.textinput_modal_data.modal_text.clone(),
                    )
                    .on_input(Message::CreateChatModalTextChange),
                )
                .foot(
                    row![
                        button(text("Cancel").horizontal_alignment(Horizontal::Center))
                            .width(Length::Fill)
                            .on_press(Message::CloseCreateChatModal),
                        {
                            let mut b = button(text("Ok").horizontal_alignment(Horizontal::Center))
                                .width(Length::Fill);
                            if !self
                                .messaging_data
                                .textinput_modal_data
                                .modal_text
                                .is_empty()
                            {
                                b = b.on_press(
                                    self.messaging_data.textinput_modal_data.message.clone(),
                                )
                            }
                            b
                        },
                    ]
                    .spacing(5),
                )
                .max_width(250.0)
                .on_close(Message::CloseCreateChatModal)
                .into()
            },
        )
        .backdrop(Message::CloseCreateChatModal)
        .on_esc(Message::CloseCreateChatModal);

        Modal::new(
            self.messaging_data.show_error_modal,
            text_input_modal,
            || {
                Card::new(
                    text("Error"),
                    text(format!(
                        "Error:{}",
                        self.messaging_data.error_message.clone()
                    ))
                    .style(Color::from_rgb(1.0, 0.0, 0.0)),
                )
                .foot(
                    button(text("Ok").horizontal_alignment(Horizontal::Center))
                        .width(Length::Fill)
                        .on_press(Message::ErrorModalClose),
                )
                .max_width(250.0)
                .on_close(Message::ErrorModalClose)
                .into()
            },
        )
        .backdrop(Message::ErrorModalClose)
        .on_esc(Message::ErrorModalClose)
        .into()
    }
}
