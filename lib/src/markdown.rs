use std::collections::HashMap;

pub struct Markdown {
    title: String,
    raw: String,
    body: String,
    frontmatter: HashMap<String, String>,
    headers: Vec<Header>,
}

pub struct Header {
    level: u8,
    text: String,
    id: String,
}
