use super::entry::*;

const ENTRIES_PATH: &str = "./src/tools/seed/data/entries.json";
const SEED_PATH: &str = "./src/tools/seed/data/seed.sql";

pub fn load_entries() -> Vec<Entry> {
    let seed = std::fs::read_to_string(ENTRIES_PATH).unwrap();
    let seed: Vec<Entry> = serde_json::from_str(&seed).unwrap();
    seed
}

pub fn save_entries(seed: Vec<Entry>) {
    let seed = serde_json::to_string(&seed).unwrap();
    std::fs::write(ENTRIES_PATH, seed).unwrap();
}

pub fn save_seed(seed: String) {
    std::fs::write(SEED_PATH, seed).unwrap();
}
