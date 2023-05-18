#![allow(unused_variables)]
use aes::{
    cipher::{
        generic_array::GenericArray, inout::InOutBuf, BlockDecrypt, BlockEncrypt, BlockSizeUser,
        KeyInit,
    },
    Aes256, Aes256Dec, Aes256Enc,
};
use rand::Rng;

pub type Key = [u8; 32];
const BLOCK_SIZE: usize = 16;

pub fn generate_aes_key() -> Key {
    let mut key = [0u8; 32]; // 256-bit key
    let mut rng = rand::thread_rng();
    rng.fill(&mut key);
    key
}

//TANK YOU CHATGPT FOR THESE BLOCKS FUNCTIONS
fn to_blocks(data: &[u8]) -> Vec<GenericArray<u8, <Aes256 as BlockSizeUser>::BlockSize>> {
    let num_blocks = (data.len() + BLOCK_SIZE - 1) / BLOCK_SIZE;

    let padded_data = data
        .iter()
        .cloned()
        .chain(std::iter::repeat(0).take(num_blocks * BLOCK_SIZE - data.len()))
        .collect::<Vec<u8>>();
    padded_data
        .chunks(BLOCK_SIZE)
        .map(|i| {
            let mut block = [0; BLOCK_SIZE];
            block[..BLOCK_SIZE].copy_from_slice(i);
            GenericArray::from(block)
        })
        .collect()
}

fn merge_blocks(
    blocks: Vec<GenericArray<u8, <Aes256 as BlockSizeUser>::BlockSize>>,
    remove_padding: bool,
) -> Vec<u8> {
    let combined_data: Vec<u8> = blocks
        .iter()
        .flat_map(|block| block.iter().copied())
        .collect();

    if remove_padding {
        let last_non_zero = combined_data
            .iter()
            .rposition(|&byte| byte != 0)
            .map(|index| index + 1)
            .unwrap_or(0);

        combined_data[..last_non_zero].to_vec()
    } else {
        combined_data
    }
}

pub fn encrypt_data(key: &Key, data: &[u8]) -> Vec<u8> {
    let cipher = Aes256Enc::new_from_slice(key).unwrap();

    let blocks = to_blocks(data);
    let mut out = Vec::new();
    out.resize(blocks.len(), GenericArray::from([0; BLOCK_SIZE]));
    let in_out = InOutBuf::new(&blocks, &mut out).unwrap();

    cipher.encrypt_blocks_inout(in_out);
    merge_blocks(out, false)
}

pub fn decrypt_data(key: &Key, data: &[u8]) -> Vec<u8> {
    let cipher = Aes256Dec::new_from_slice(key).unwrap();

    let blocks = to_blocks(data);
    let mut out = Vec::new();
    out.resize(blocks.len(), GenericArray::from([0; BLOCK_SIZE]));
    let in_out = InOutBuf::new(&blocks, &mut out).unwrap();

    cipher.decrypt_blocks_inout(in_out);
    merge_blocks(out, true)
}

#[test]
fn test_encrypt() {
    let key = generate_aes_key();
    let data = "Hello world!";

    let encrypted_data = encrypt_data(&key, data.as_bytes());
    assert_ne!(encrypted_data.len(), 0);
}

#[test]
fn test_decrypt() {
    let key = generate_aes_key();
    let data = "Hello world!";

    let encrypted_data = encrypt_data(&key, data.as_bytes());
    assert_ne!(encrypted_data.len(), 0);
    println!("Encrypted message length: {}", encrypted_data.len());

    let decrypted = String::from_utf8(decrypt_data(&key, &encrypted_data)).unwrap();
    assert_eq!(data, decrypted);
}

#[test]
fn test_encrypt_decrypt_long() {
    //Taking the BIGGEST file
    let data = include_str!("../../frontend/src/messaging.rs");
    let key = generate_aes_key();

    let encrypted_msg = encrypt_data(&key, data.as_bytes());
    assert_ne!(encrypted_msg.len(), 0);
    println!("Encrypted message length: {}", encrypted_msg.len());

    let decrypted = String::from_utf8(decrypt_data(&key, &encrypted_msg)).unwrap();
    assert_eq!(data, decrypted);
}
