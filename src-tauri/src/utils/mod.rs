use rand::{Rng, thread_rng};

pub fn generate_random_hash(num: i32) -> String {
    let random_bytes: Vec<u8> = (0..num).map(|_| thread_rng().gen::<u8>()).collect();
    hex::encode(random_bytes)
}