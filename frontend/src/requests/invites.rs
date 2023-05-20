use common_lib::SendInvite;

use crate::{
    grimoire, main_window::MainForm, time_utils::naive_utc_to_naive_local, CLIENT, COOKIE_STORE,
};

impl MainForm {
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
        self.messaging_data.invites.iter_mut().for_each(|i| {
            i.created_at = naive_utc_to_naive_local(&i.created_at);
        });
    }
}
