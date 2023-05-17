#![allow(unused)]
use diesel::{update, ExpressionMethods, QueryDsl, RunQueryDsl};
use rand::Rng;

use crate::DbPool;

pub type Key = [u8; 32];

fn generate_aes_key() -> Key {
    let mut key = [0u8; 32]; // 256-bit key
    let mut rng = rand::thread_rng();
    rng.fill(&mut key);
    key
}

//Generates new keys for all group chats, overrides the old ones
//DO NOT USE IF THERE ARE ENCRYPTED MESSAGES IN THE DATABASE
//THERE WILL BE NO WAY TO RECOVER OLD KEYS
fn generate_keys(pool: DbPool) {
    use crate::schema::group_chats::dsl::*;
    let connection = &mut pool.get().unwrap();

    let ids: Vec<i32> = group_chats.select(chat_id).load(connection).unwrap();
    let ids = ids.iter().map(|i| (i, generate_aes_key()));
    //Kinda jank but should work
    for i in ids {
        update(group_chats)
            .filter(chat_id.eq(i.0))
            .set(key.eq::<String>(i.1.iter().map(|i| *i as char).collect()))
            .execute(connection)
            .unwrap();
    }
}
