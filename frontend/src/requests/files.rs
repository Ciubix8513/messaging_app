use common_lib::{
    encryption::{encrypt_string_to_base64, read_and_encrypt_file},
    UploadFile,
};
use reqwest::blocking::multipart::{Form, Part};

use crate::{grimoire, main_window::MainForm, CLIENT, COOKIE_STORE};

impl MainForm {
    pub fn upload_files(&mut self) {
        let metadata = UploadFile {
            chat_id: self.messaging_data.selected_chat.unwrap(),
            message_text: encrypt_string_to_base64(
                &self.messaging_data.current_message.clone(),
                &self.messaging_data.chat_key,
            ),
        };
        let mut form = Form::new().text(
            common_lib::grimoire::UPLOAD_METADATA_NAME,
            serde_json::to_string(&metadata).unwrap(),
        );
        for i in &self.messaging_data.attachments {
            form = form.part(
                i.file_name().unwrap().to_str().unwrap().to_string(),
                Part::bytes(read_and_encrypt_file(i, &self.messaging_data.chat_key).unwrap()),
            );
        }
        let result = CLIENT
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .post(grimoire::FILES_UPLOAD.clone())
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
            .multipart(form)
            .send()
            .unwrap();
        if !result.status().is_success() {
            let status = result.status();
            self.error_message(result.text().unwrap(), status);
            return;
        }
        self.messaging_data.current_message.clear();
        self.messaging_data.attachments.clear();
        self.load_messages(false);
    }
}
