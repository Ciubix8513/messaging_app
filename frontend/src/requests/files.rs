use std::thread;

use common_lib::{
    encryption::{decrypt_data, encrypt_string_to_base64, read_and_encrypt_file},
    UploadFile,
};
use reqwest::blocking::multipart::{Form, Part};

use crate::{grimoire, main_window::MainForm, ADDITIONAL_CLIENT, COOKIE_STORE};

//Using async clients here cause files can take A LONG time and I don't want to prevent messages
//from loading while file is being up/downloaded
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
        let data = self.messaging_data.attachments.clone();
        let key = self.messaging_data.chat_key;

        thread::spawn(move || {
            for i in data {
                form = form.part(
                    i.file_name().unwrap().to_str().unwrap().to_string(),
                    Part::bytes(read_and_encrypt_file(&i, &key).unwrap()),
                );
            }
            let client = ADDITIONAL_CLIENT.lock().unwrap().as_ref().unwrap().clone();
            client
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
        });
        self.messaging_data.current_message.clear();
        self.messaging_data.attachments.clear();
        self.load_messages(false);
    }
    pub fn download_file(&mut self, id: i32, filename: &str) {
        let path = rfd::FileDialog::new().set_file_name(filename).save_file();
        if path.is_none() {
            return;
        }
        let path = path.unwrap();
        let key = self.messaging_data.chat_key;
        thread::spawn(move || {
            let client = ADDITIONAL_CLIENT.lock().unwrap().as_ref().unwrap().clone();
            let result = client
                .get(grimoire::FILES_DOWNLOAD.clone())
                .query(&[("id", id)])
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
            let bytes = result.bytes().unwrap();
            let bytes = decrypt_data(&key, &bytes);
            std::fs::write(path, bytes).unwrap();
        });
    }
}
