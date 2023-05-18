use rand::Rng;
pub type Key = [u8; 32];

pub fn generate_aes_key() -> Key {
    let mut key = [0u8; 32]; // 256-bit key
    let mut rng = rand::thread_rng();
    rng.fill(&mut key);
    key
}
