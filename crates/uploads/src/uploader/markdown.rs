use crate::error::{Error, Result};
use regex::Regex;

pub struct Markdown {
    pub title: String,
    pub body: String,
}

impl Markdown {
    pub fn from_file(path: String) -> Result<Self> {
        let (title, body) = Self::title(
            std::fs::read_to_string(&path).map_err(|e| Error::FileParseError(e.to_string()))?,
        );

        Ok(Self { title, body })
    }

    fn title(body: String) -> (String, String) {
        let re = Regex::new(r"^# (.*)").unwrap();
        match re.captures(&body) {
            Some(caps) => {
                let title = caps.get(1).unwrap().as_str().to_owned();
                let body = re.replace(&body, "").to_string();

                (title, body)
            }
            None => ("Untitled".to_owned(), body),
        }
    }
}

pub fn file_is_markdown(file: &str) -> bool {
    file.ends_with(".md")
}
