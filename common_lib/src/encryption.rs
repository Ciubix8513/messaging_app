use crypto::{
    aes, blockmodes,
    buffer::{RefReadBuffer, RefWriteBuffer},
};
use rand::Rng;
pub type Key = [u8; 32];

pub fn generate_aes_key() -> Key {
    let mut key = [0u8; 32]; // 256-bit key
    let mut rng = rand::thread_rng();
    rng.fill(&mut key);
    key
}
//
//Encrypts a message with a key
pub fn encrypt_message(key: Key, message: String) -> Vec<u8> {
    //Using ecb for better parallelization, cause I don't think I need the extra security of cbc
    //Need to add padding bc messages can be of wrong length
    let mut cipher = aes::ecb_encryptor(aes::KeySize::KeySize256, &key, blockmodes::PkcsPadding);

    let mut out = Vec::new();
    out.resize(message.len(), 0 as u8);
    let mut write_buf = RefWriteBuffer::new(&mut out);
    let mut read_buf = RefReadBuffer::new(message.as_bytes());

    cipher.encrypt(&mut read_buf, &mut write_buf, true).unwrap();

    out
}

#[test]
fn test_encrypt() {
    let key = generate_aes_key();
    let msg = "Hello world!".to_string();

    let encrypted_msg = encrypt_message(key, msg);
    assert_ne!(encrypted_msg.len(), 0);
}
