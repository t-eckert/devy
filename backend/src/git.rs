use std::fs;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Git {
    bin: String,
}

impl Git {
    pub fn new(bin: String) -> Result<Self, String> {
        if !fs::metadata(&bin).is_ok() {
            return Err(format!("Git binary not found at {}", bin));
        }

        Ok(Self { bin })
    }

    pub fn clone_repo(self, url: &str) -> Result<(), String> {
        let output = std::process::Command::new(&self.bin)
            .arg("clone")
            .arg(format!("https://{}", url))
            .arg(format!("/tmp/{}", Uuid::new_v4().to_string()))
            .output()
            .expect("Failed to execute git clone");
        dbg!(output);

        Ok(())
    }
}
