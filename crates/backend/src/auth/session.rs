use rand::{distributions::Alphanumeric, Rng};

const KEY_SIZE: usize = 32;

pub fn generate_encoding_key() -> String {
    let mut rng = rand::thread_rng();
    let key: String = rng
        .sample_iter(&Alphanumeric)
        .take(KEY_SIZE)
        .map(char::from)
        .collect();

    key
}
