use regex::Regex;

pub fn file_is_markdown(file: &str) -> bool {
    file.ends_with(".md")
}

pub fn title(body: &str) -> String {
    let re = Regex::new(r"^# (.*)").unwrap();
    match re.captures(body) {
        Some(caps) => caps.get(1).unwrap().as_str().to_owned(),
        None => "Untitled".to_owned(),
    }
}
