#![allow(unused)]
use std::collections::HashMap;

use base64::Engine;
use common_lib::encryption::{
    decrypt_key, encrypt_data, encrypt_key, generate_aes_key, into_key, Key, ENCODING_ENGINE,
};
use diesel::{update, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::DbPool;

//Encrypts the chat keys
//If the old key is not provided assumes that the messages were not encrypted previously and
//generates chat keys and encrypts all the messages
pub fn deploy(new_key: Key, old_key: Option<Key>, pool: &DbPool) {
    match old_key {
        Some(key) => {
            println!("Re encrypting the keys in the database");
            reencrypt_keys(pool, new_key, key)
        }
        //Assume the db wasn't previously encrypted and contains plain text messages
        None => {
            println!("No key found, assuming the databse wasn't encrypted previously");
            let res = dialoguer::Confirm::new()
                .with_prompt("Encrypt the database?\nWARNING if the database was encrypted previously, this will effectively DELETE ALL MESSAGES\nContinue?")
                .show_default(false)
                .interact()
                .unwrap();
            if !res {
                panic!("No old key provided")
            }
            println!("Encrypting all existing messages in the database");
            generate_keys(pool, new_key);
            encrypt_existing_messages(pool, new_key);
        }
    }
}

//Re encrypts chat keys using the new deployment key
fn reencrypt_keys(pool: &DbPool, new_key: Key, old_key: Key) {
    use crate::schema::group_chats::dsl::*;
    let connection = &mut pool.get().unwrap();

    let keys: Vec<(String, i32)> = group_chats.select((key, chat_id)).load(connection).unwrap();

    let keys = keys.iter().map(|i| {
        (
            i.1,
            encrypt_key(
                &decrypt_key(&into_key(&ENCODING_ENGINE.decode(&i.0).unwrap()), &old_key),
                &new_key,
            ),
        )
    });

    for i in keys {
        update(group_chats)
            .filter(chat_id.eq(i.0))
            .set(key.eq(ENCODING_ENGINE.encode(i.1)))
            .execute(connection)
            .unwrap();
    }
}

//Generates new keys for all group chats, overrides the old ones
//DO NOT USE IF THERE ARE ENCRYPTED MESSAGES IN THE DATABASE
//THERE WILL BE NO WAY TO RECOVER OLD KEYS
fn generate_keys(pool: &DbPool, encryption_key: Key) {
    use crate::schema::group_chats::dsl::*;

    let connection = &mut pool.get().unwrap();

    let ids: Vec<i32> = group_chats.select(chat_id).load(connection).unwrap();

    let ids = ids.iter().map(|i| {
        (i, {
            let unencrypted_key = generate_aes_key();
            encrypt_key(&unencrypted_key, &encryption_key)
        })
    });

    //Kinda jank but should work
    for i in ids {
        update(group_chats)
            .filter(chat_id.eq(i.0))
            .set(key.eq(ENCODING_ENGINE.encode(i.1)))
            .execute(connection)
            .unwrap();
    }
}

//Encrypts all messages in the database with their chat keys
fn encrypt_existing_messages(pool: &DbPool, encryption_key: Key) {
    use crate::schema::group_chats::dsl as gc;
    use crate::schema::messages::dsl as msgs;
    let connection = &mut pool.get().unwrap();

    let messages: Vec<(String, i32, i32)> = msgs::messages
        .inner_join(gc::group_chats)
        .select((msgs::message_text, msgs::chat_id, msgs::message_id))
        .load(connection)
        .unwrap();

    let keys: Vec<(String, i32)> = gc::group_chats
        .select((gc::key, gc::chat_id))
        .load(connection)
        .unwrap();

    let keys = keys
        .iter()
        .map(|i| {
            (
                i.1,
                decrypt_key(
                    &into_key(&ENCODING_ENGINE.decode(&i.0).unwrap()),
                    &encryption_key,
                ),
            )
        })
        .collect::<HashMap<i32, Key>>();

    let messages = messages.iter().map(|i| {
        let key = keys[&i.1];
        let encrypted_message = ENCODING_ENGINE.encode(encrypt_data(&key, i.0.as_bytes()));
        (encrypted_message, i.2)
    });

    for i in messages {
        update(msgs::messages)
            .filter(msgs::message_id.eq(i.1))
            .set(msgs::message_text.eq(i.0))
            .execute(connection)
            .unwrap();
    }
}
