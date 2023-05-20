use common_lib::{encryption::into_key, GetChat};
use reqwest::Method;

use crate::{grimoire, main_window::MainForm, window_structs::Chat, CLIENT, COOKIE_STORE};

impl MainForm {
    pub fn get_chat_key(&mut self) {
        let result = CLIENT
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .get(grimoire::CHATS_GET_KEY.clone())
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
            return;
        }
        let encrypted_key = result.json::<Vec<u8>>().unwrap();
        let k = self
            .messaging_data
            .key
            .clone()
            .unwrap()
            .decrypt(rsa::Pkcs1v15Encrypt, &encrypted_key)
            .unwrap();

        self.messaging_data.chat_key = into_key(&k);
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

    pub fn update_chat_list(&mut self) {
        let result = CLIENT
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .get(grimoire::CHATS_GET.clone())
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
        let data: Vec<GetChat> = result.json().unwrap_or_default();
        self.messaging_data.chats = data
            .iter()
            .map(|i| Chat {
                chat_id: i.chat_id,
                chat_name: i.chat_name.clone(),
            })
            .collect();
    }
}
