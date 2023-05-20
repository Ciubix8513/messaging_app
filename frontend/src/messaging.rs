use iced::{
    alignment::{self, Horizontal},
    theme::Container,
    widget::{button, column, container, row, scrollable, text, text_input, Column},
    Color, Length, Padding,
};
use iced_aw::{Card, Modal};
use reqwest::StatusCode;

use crate::{
    grimoire,
    main_window::{MainForm, Message, SCROLLABLE_ID},
    window_structs::MessageViewMode,
};

impl MainForm {
    pub fn error_message(&mut self, message: String, code: StatusCode) {
        println!("ERROR {:#?} / {}", code, message);
        self.messaging_data.show_error_modal = true;
        self.messaging_data.error_message = message;
    }

    fn side_bar(&self) -> iced::Element<'_, Message> {
        // let contents = text("").height(Length::Fill);
        let contents = self
            .messaging_data
            .chats
            .iter()
            .map(|t| button(text(&t.chat_name)).on_press(Message::SelectChat(t.chat_id)))
            .fold(Column::new(), Column::push)
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
        .into()
    }

    fn top_bar(&self) -> iced::Element<'_, Message> {
        //Start with the top bar
        let logout = button("Logout").on_press(Message::LogoutButtonPressed);
        let invites = button("Invites").on_press(Message::InvitesButtonPressed);

        container(row![invites, logout].spacing(2))
            .align_x(alignment::Horizontal::Right)
            .height(35)
            .padding(2)
            .width(Length::Fill)
            .style(Container::Box)
            .into()
    }

    fn messaging_view_mode(&self) -> iced::widget::Container<'_, Message> {
        if self.messaging_data.selected_chat.is_some() {
            container(
                column![
                    scrollable(
                        self.messaging_data
                            .messages
                            .iter()
                            .map(|i| {
                                let uname = text(&i.username);
                                let date = text(i.sent_at).style(grimoire::DATE_COLOR.clone());
                                let body = text(&i.message_text);
                                container(column![row![uname, date].spacing(5), body])
                            })
                            .fold(Column::new(), Column::push)
                            .spacing(10)
                            .padding(Padding {
                                right: 20.0,
                                bottom: 10.0,
                                left: 0.0,
                                top: 0.0
                            }),
                    )
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .id(SCROLLABLE_ID.clone()),
                    {
                        let mut input = text_input("...", &self.messaging_data.current_message)
                            .on_input(Message::MessageEdited);
                        if !self.messaging_data.current_message.is_empty() {
                            input = input.on_submit(Message::SendMessage);
                        }
                        let mut send_button = button("Send");
                        if !self.messaging_data.current_message.is_empty() {
                            send_button = send_button.on_press(Message::SendMessage);
                        }
                        row![input, send_button].spacing(5)
                    }
                ]
                .spacing(2),
            )
        } else {
            //Kinda hacky but it's fine
            container(text(""))
        }
    }

    fn invites_view_mode(&self) -> iced::widget::Container<'_, Message> {
        container(
            column![
                text("Invites:"),
                if self.messaging_data.invites.is_empty() {
                    scrollable(text("None"))
                } else {
                    scrollable(
                        self.messaging_data
                            .invites
                            .iter()
                            .map(|i| {
                                container({
                                    let c_name = text(i.chat_name.clone());
                                    let s_name = text(format!("from: {}", i.sender_name.clone()));
                                    let date =
                                        text(i.created_at).style(grimoire::DATE_COLOR.clone());
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
                            .fold(Column::new(), Column::push),
                    )
                }
            ]
            .spacing(10),
        )
    }

    pub fn messaging_view(&self) -> iced::Element<'_, Message> {
        let top_bar = self.top_bar();
        let side_bar = self.side_bar();
        let main_view = match self.messaging_data.mode {
            MessageViewMode::Messages => self.messaging_view_mode(),
            MessageViewMode::Invites => self.invites_view_mode(),
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
                                );
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
