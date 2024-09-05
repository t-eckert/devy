pub struct Email {
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    pub body: String,
}

impl Email {
    pub fn new(from: String, to: Vec<String>, subject: String, body: String) -> Self {
        Self {
            from,
            to,
            subject,
            body,
        }
    }
}
