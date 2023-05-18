#![allow(unused)]
use std::collections::HashMap;

use common_lib::encryption::{
    decrypt_key, encrypt_data, encrypt_key, generate_aes_key, into_key, Key,
};
use diesel::{update, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::DbPool;

//Encrypts the chat keys
//If the old key is not provided assumes that the messages were not encrypted previously and
//generates chat keys and encrypts all the messages
fn deploy(new_key: Key, old_key: Option<Key>) {}

//Generates new keys for all group chats, overrides the old ones
//DO NOT USE IF THERE ARE ENCRYPTED MESSAGES IN THE DATABASE
//THERE WILL BE NO WAY TO RECOVER OLD KEYS
fn generate_keys(pool: DbPool, encryption_key: Key) {
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
            .set(key.eq::<String>(i.1.iter().map(|i| *i as char).collect()))
            .execute(connection)
            .unwrap();
    }
}

//Encrypts all messages in the database with their chat keys
fn encrypt_existing_messages(pool: DbPool, encryption_key: Key) {
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
        .map(|i| (i.1, decrypt_key(&into_key(i.0.as_bytes()), &encryption_key)))
        .collect::<HashMap<i32, Key>>();

    let messages = messages.iter().map(|i| {
        let key = keys[&i.1];
        let encrypted_message =
            unsafe { String::from_utf8_unchecked(encrypt_data(&key, i.0.as_bytes())) };
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
