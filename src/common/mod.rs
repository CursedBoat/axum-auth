use rand::rngs::OsRng;
use rand::{Rng, RngCore};

pub mod database;
pub mod structs;
pub mod error;

pub fn generate_random_bytes() -> [u8; 16] {
    let mut rng = rand::thread_rng(); // Get a random number generator
    rng.gen::<[u8; 16]>() // Generate a random 16-byte array
}

pub fn generate_uuid() -> String {
    let uuid = uuid::Uuid::new_v4();
    uuid.to_string()
}

pub fn generate_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.try_fill_bytes(&mut bytes).unwrap();
    let random_hex: String = bytes.iter().map(|byte| format!("{:02x}", byte)).collect();

    return random_hex;
}
