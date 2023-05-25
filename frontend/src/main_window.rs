#![allow(clippy::enum_variant_names)]
use std::{
    os::unix::prelude::MetadataExt,
    path::PathBuf,
    time::{Duration, Instant},
};

use crate::{
    grimoire,
    window_structs::{LoginData, MessageViewMode, MessagingData, SignupData},
};
use iced::{executor, widget::scrollable, Application, Command, Subscription};
use once_cell::sync::Lazy;

#[derive(Default, PartialEq, Eq)]
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

#[derive(Default, Debug, Clone)]
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
    #[default]
    ConfirmCreateChat,
    CreateChatModalTextChange(String),
    ErrorModalClose,
    SelectChat(i32),
    InviteButtonPressed,
    ConfirmInvite,
    InvitesButtonPressed,
    DeclineInvite(i32),
    AcceptInvite(i32),
    MessageEdited(String),
    SendMessage,
    RefreshMessages(Instant),
    AttachFile,
    RemoveFile(PathBuf),
    ClickFile(i32),
}

pub static SCROLLABLE_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);

impl Application for MainForm {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Login")
    }

    #[allow(clippy::too_many_lines)]
    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            //Login stuff
            Message::LoginChanged(v) => self.login_data.login_textbox = v,
            Message::PasswordChanged(v) => self.login_data.password_textbox = v,
            Message::LoginButtonPressed => self.login(),
            Message::LoginViewSignupButtonPressed => {
                //Clear signup data
                self.signup_data = SignupData::default();
                self.winodow_mode = WindowMode::SignUp;
            }
            Message::BackButtonPressed => self.winodow_mode = WindowMode::Login,
            //Signup stuff
            Message::UsernameChanged(v) => self.signup_data.username_textbox = v,
            Message::EmailChanged(v) => self.signup_data.email_textbox = v,
            Message::SignupPasswordChanged(v, i) => self.signup_data.password_textbox[i] = v,
            Message::SignupButtonPressed => self.signup(),
            Message::LogoutButtonPressed => {
                self.logout();
                //Clear data just in case
                self.messaging_data = MessagingData::default();
            }
            Message::CreateChatButtonPressed => {
                self.messaging_data.textinput_modal_data.modal_text.clear();
                self.messaging_data.textinput_modal_data.title = "Create chat".to_string();
                self.messaging_data.textinput_modal_data.placeholder = "Chat name".to_string();
                self.messaging_data.textinput_modal_data.show_modal = true;
                self.messaging_data.textinput_modal_data.message = Message::ConfirmCreateChat;
            }
            Message::CloseCreateChatModal => {
                self.messaging_data.textinput_modal_data.show_modal = false;
            }
            Message::ConfirmCreateChat => {
                self.create_chat();
                self.messaging_data.textinput_modal_data.show_modal = false;
            }
            Message::CreateChatModalTextChange(v) => {
                self.messaging_data.textinput_modal_data.modal_text = v;
            }
            Message::ErrorModalClose => self.messaging_data.show_error_modal = false,
            Message::SelectChat(val) => {
                self.messaging_data.mode = MessageViewMode::Messages;
                self.messaging_data.selected_chat = Some(val);
                self.get_chat_key();
                self.messaging_data.messages.clear();
                self.load_messages(true);
                return scrollable::snap_to(SCROLLABLE_ID.clone(), scrollable::RelativeOffset::END);
            }
            Message::InviteButtonPressed => {
                self.messaging_data.textinput_modal_data.modal_text.clear();
                self.messaging_data.textinput_modal_data.title = "Invite to the chat".to_string();
                self.messaging_data.textinput_modal_data.placeholder = "Username".to_string();
                self.messaging_data.textinput_modal_data.show_modal = true;
                self.messaging_data.textinput_modal_data.message = Message::ConfirmInvite;
            }
            Message::ConfirmInvite => {
                self.send_invite();
                self.messaging_data.textinput_modal_data.show_modal = false;
            }
            Message::InvitesButtonPressed => {
                self.messaging_data.selected_chat = None;
                self.messaging_data.mode = MessageViewMode::Invites;
                self.update_invites_list();
            }
            Message::DeclineInvite(id) => {
                self.decline_invite(id);
                self.update_chat_list();
                self.update_invites_list();
            }
            Message::AcceptInvite(id) => {
                self.accept_invite(id);
                self.update_chat_list();
                self.update_invites_list();
            }
            Message::MessageEdited(val) => self.messaging_data.current_message = val,
            Message::SendMessage => {
                if self.messaging_data.attachments.is_empty() {
                    self.send_message();
                } else {
                    self.upload_files();
                }
                return scrollable::snap_to(SCROLLABLE_ID.clone(), scrollable::RelativeOffset::END);
            }
            Message::RefreshMessages(..) => {
                if self.load_messages(false) {
                    return scrollable::snap_to(
                        SCROLLABLE_ID.clone(),
                        scrollable::RelativeOffset::END,
                    );
                }
            }
            Message::AttachFile => {
                let files = rfd::FileDialog::new().pick_files();
                if files.is_none() {
                    return Command::none();
                }
                let files = files.unwrap();
                let mut is_over = false;
                //Discard all files
                let files = files.iter().cloned().filter(|i| {
                    let over = std::fs::metadata(i).unwrap().size() > grimoire::MAX_FILESIZE;
                    is_over = over || is_over;
                    !over
                });
                self.messaging_data.attachments = files.collect();
                if is_over {
                    self.messaging_data.error_message = "Files must be under 64 MB".to_string();
                    self.messaging_data.show_error_modal = true;
                }
            }
            Message::ClickFile(_) => todo!(),
            Message::RemoveFile(f) => self.messaging_data.attachments.retain(|i| i != &f),
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        match self.winodow_mode {
            WindowMode::Login => self.login_view(),
            WindowMode::SignUp => self.signup_view(),
            WindowMode::Messaging => self.messaging_view(),
        }
    }

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        if self.winodow_mode != WindowMode::Messaging || self.messaging_data.selected_chat.is_none()
        {
            return Subscription::none();
        }
        iced::time::every(Duration::from_secs(grimoire::REFRESH_TIME)).map(Message::RefreshMessages)
    }

    type Executor = executor::Default;

    type Theme = iced::Theme;

    type Flags = ();
}
