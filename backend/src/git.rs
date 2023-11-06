#[derive(Debug, Clone)]
pub struct Git {
    bin: String,
}

impl Git {
    pub fn new(bin: String) -> Self {
        Self { bin }
    }

    pub fn clone_repo(self, url: &str) -> Result<(), String> {
        Ok(())
    }
}
