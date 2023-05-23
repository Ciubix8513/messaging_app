use common_lib::{
    encryption::{decrypt_base64_to_string, encrypt_string_to_base64},
    GetMessage, SendMessage,
};

use crate::{
    grimoire, main_window::MainForm, time_utils::naive_utc_to_naive_local, CLIENT, COOKIE_STORE,
};

impl MainForm {
    pub fn send_message(&mut self) {
        let body = SendMessage {
            chat_id: self.messaging_data.selected_chat.unwrap(),
            text: encrypt_string_to_base64(
                &self.messaging_data.current_message.clone(),
                &self.messaging_data.chat_key,
            ),
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
        self.load_messages(false);
    }

    pub fn load_messages(&mut self, force: bool) -> bool {
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
        if new.len() == self.messaging_data.messages.len() && !force {
            return false;
        }

        let new = new[self.messaging_data.messages.len()..]
            .iter()
            .map(|i| GetMessage {
                message_id: i.message_id,
                user_id: i.user_id,
                username: i.username.clone(),
                message_text: decrypt_base64_to_string(
                    &i.message_text,
                    &self.messaging_data.chat_key,
                ),
                sent_at: naive_utc_to_naive_local(&i.sent_at),
                files: Vec::new()
            });
        self.messaging_data.messages.append(&mut new.collect());

        true
    }
}
