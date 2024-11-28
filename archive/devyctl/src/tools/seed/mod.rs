mod entry;
mod formatting;
mod generation;
mod io;

use entry::Entry;
use lib::openai;

#[derive(Debug)]
pub struct Record {
    pub name: String,
    pub blog_name: String,
    pub blog_subject: String,
    pub post_count: usize,
}

/// Formats the entries into a seed file.
pub async fn format() -> Result<(), anyhow::Error> {
    let entries = io::load_entries();
    let seed = entries
        .into_iter()
        .map(|entry| formatting::format_entry(entry))
        .collect::<Vec<String>>()
        .join("\n");
    io::save_seed(seed);

    Ok(())
}

/// Generates a new entry and saves it to the entries file.
pub async fn generate(openai_api_key: &str, record: Record) -> Result<(), anyhow::Error> {
    let openai_client = openai::Client::new(openai_api_key);
    let entry = Entry::generate(openai_client, record).await?;
    let mut entries = io::load_entries();
    entries.push(entry);
    io::save_entries(entries);

    Ok(())
}
