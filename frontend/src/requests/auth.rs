use common_lib::{AddUser, Login, UserData};
use rsa::pkcs8::DecodePrivateKey;

use crate::{
    grimoire,
    main_window::{MainForm, WindowMode},
    window_structs::LoginData,
    CLIENT, COOKIE_STORE,
};

impl MainForm {
    pub fn signup(&mut self) {
        if self.signup_data.email_textbox.is_empty()
            || self.signup_data.username_textbox.is_empty()
            || self.signup_data.password_textbox[0].is_empty()
        {
            self.signup_data.show_error_message = true;
            self.signup_data.error_message = "All fields must be filled in".to_string();
            return;
        }
        if self.signup_data.password_textbox[0] != self.signup_data.password_textbox[1] {
            self.signup_data.show_error_message = true;
            self.signup_data.error_message = "Passwords don't match".to_string();
            return;
        }
        if self.signup_data.password_textbox[0].len() < 8 {
            self.signup_data.show_error_message = true;
            self.signup_data.error_message =
                "Password must be at least 8 characters long".to_string();
            return;
        }
        //Validate email
        let re = crate::regex::email();
        if !re.is_match(&self.signup_data.email_textbox) {
            self.signup_data.show_error_message = true;
            self.signup_data.error_message = "Invalid email adress".to_string();
            return;
        }
        //Passed all the checks, do the request
        let body = AddUser {
            username: self.signup_data.username_textbox.clone(),
            password: self.signup_data.password_textbox[0].clone(),
            email: self.signup_data.email_textbox.clone(),
        };
        let response = CLIENT
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .post(grimoire::USERS_ADD_USER.clone())
            .json(&body)
            .send()
            .unwrap();
        if !response.status().is_success() {
            self.signup_data.show_error_message = true;
            self.signup_data.error_message = response.text().unwrap();
            return;
        }
        let body = Login {
            username: self.signup_data.username_textbox.clone(),
            password: self.signup_data.password_textbox[0].clone(),
        };
        let response = CLIENT
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .post(grimoire::AUTH_LOGIN.clone())
            .json(&body)
            .send()
            .unwrap()
            .json::<UserData>()
            .unwrap();
        self.messaging_data.key =
            Some(rsa::RsaPrivateKey::from_pkcs8_pem(&response.private_key).unwrap());

        //Log in
        self.winodow_mode = WindowMode::Messaging;
        self.update_chat_list();
    }

    pub fn login(&mut self) {
        let body = Login {
            username: self.login_data.login_textbox.clone(),
            password: self.login_data.password_textbox.clone(),
        };
        let response = CLIENT
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .post(grimoire::AUTH_LOGIN.clone())
            .json(&body)
            .send()
            .unwrap();

        if !response.status().is_success() {
            self.login_data.error_message = String::from("Wrong username or password");
            self.login_data.show_error_message = true;
            self.login_data.password_textbox.clear();
            return;
        }
        let response = response.json::<UserData>().unwrap();

        self.messaging_data.key =
            Some(rsa::RsaPrivateKey::from_pkcs8_pem(&response.private_key).unwrap());
        //Login
        self.winodow_mode = WindowMode::Messaging;
        self.update_chat_list();
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
        let result = CLIENT
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .delete(grimoire::AUTH_LOGOUT.clone())
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
}
