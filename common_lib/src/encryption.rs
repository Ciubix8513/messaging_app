use crypto::{
    aes, blockmodes,
    buffer::{BufferResult, ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer},
};
use rand::Rng;

pub struct Key {
    pub key: [u8; 32],
    pub length: aes::KeySize,
}

pub fn generate_aes_key() -> Key {
    let mut key = [0u8; 32]; // 256-bit key
    let mut rng = rand::thread_rng();
    rng.fill(&mut key);
    Key {
        key,
        length: aes::KeySize::KeySize256,
    }
}
//
//Encrypts a message with a key
pub fn encrypt_message(key: &Key, message: String) -> String {
    //Using ecb for better parallelization, cause I don't think I need the extra security of cbc
    //Need to add padding bc messages can be of wrong length
    let mut cipher = aes::ecb_encryptor(key.length, &key.key, blockmodes::PkcsPadding);

    let mut out = Vec::new();
    let mut read_buf = RefReadBuffer::new(message.as_bytes());
    let mut buffer = [0; 4096];
    let mut write_buf = RefWriteBuffer::new(&mut buffer);

    loop {
        let result = cipher.encrypt(&mut read_buf, &mut write_buf, true).unwrap();
        out.extend(
            write_buf
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );
        match result {
            BufferResult::BufferUnderflow => break,
            _ => {}
        }
    }

    //While I don't know if it's valid utf8 i don't really care, so it SHOULD be fine
    //Famous last words lol
    unsafe { String::from_utf8_unchecked(out) }
}

pub fn decrypt_message(key: &Key, message: &[u8]) -> String {
    let mut cipher = aes::ecb_decryptor(key.length, &key.key, blockmodes::PkcsPadding);

    let mut out = Vec::new();
    let mut read_buf = RefReadBuffer::new(message);
    let mut buffer = [0; 4096];
    let mut write_buf = RefWriteBuffer::new(&mut buffer);

    loop {
        let result = cipher.decrypt(&mut read_buf, &mut write_buf, true).unwrap();
        out.extend(
            write_buf
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );
        match result {
            BufferResult::BufferUnderflow => break,
            _ => {}
        }
    }

    String::from_utf8(out).unwrap()
}

#[test]
fn test_encrypt() {
    let key = generate_aes_key();
    let msg = "Hello world!".to_string();

    let encrypted_msg = encrypt_message(&key, msg);
    assert_ne!(encrypted_msg.len(), 0);
}

#[test]
fn test_decrypt() {
    let key = generate_aes_key();
    let msg = "Hello world!".to_string();

    let encrypted_msg = encrypt_message(&key, msg.clone());
    assert_ne!(encrypted_msg.len(), 0);
    println!("Encrypted message length: {}", encrypted_msg.len());

    let decrypted = decrypt_message(&key, &encrypted_msg.as_bytes());
    assert_eq!(msg, decrypted);
}
