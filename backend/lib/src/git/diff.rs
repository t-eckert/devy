use crate::git::Git;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub type Diffs = Vec<Diff>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum Diff {
    Added(String),
    Deleted(String),
    Modified(String),
    Renamed(String, String),
}

impl Diff {
    pub fn from_raw(raw: String) -> Diffs {
        let mut diffs = Vec::new();

        for line in raw.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            match parts.first() {
                // Added
                Some(&"A") => {
                    let to = parts.get(1).unwrap();
                    diffs.push(Diff::Added(to.to_string()));
                }
                // Deleted
                Some(&"D") => {
                    let from = parts.get(1).unwrap();
                    diffs.push(Diff::Deleted(from.to_string()));
                }
                // Modified
                Some(&"M") => {
                    let to = parts.get(1).unwrap();
                    diffs.push(Diff::Modified(to.to_string()));
                }
                // Renamed
                Some(&"R") => {
                    let from = parts.get(1).unwrap();
                    let to = parts.get(2).unwrap();
                    diffs.push(Diff::Renamed(from.to_string(), to.to_string()));
                }
                _ => continue,
            };
        }

        diffs
    }
}

impl Display for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Diff::Added(file) => write!(f, "Added: {}", file),
            Diff::Deleted(file) => write!(f, "Deleted: {}", file),
            Diff::Modified(file) => write!(f, "Modified: {}", file),
            Diff::Renamed(from, to) => write!(f, "Renamed: {} -> {}", from, to),
        }
    }
}

fn filter(diff: Vec<Diff>) -> Vec<Diff> {
    diff.into_iter()
        .filter(|d| match d {
            Diff::Added(file) => file.ends_with(".md"),
            Diff::Deleted(file) => file.ends_with(".md"),
            Diff::Modified(file) => file.ends_with(".md"),
            Diff::Renamed(_, to) => to.ends_with(".md"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff() {
        let raw = "M	crates/Cargo.lock
        M	crates/Cargo.toml
        M	crates/api/Cargo.toml
        M	crates/store/Cargo.toml
        M	crates/store/src/lib.rs
        D	crates/upload/Cargo.toml
        D	crates/upload/src/lib.rs
        D	crates/upload/src/uploader.rs
        A	crates/uploads/Cargo.toml
        R071	crates/upload/src/error.rs	crates/uploads/src/error.rs
        A	crates/uploads/src/git.rs
        A	crates/uploads/src/lib.rs
        A	crates/uploads/src/new_upload.rs
        A	crates/uploads/src/uploader.rs
        A	notebook/tasks/timing-bug.md
        M	notebook/wiki/Uploads.md
        M	site/src/lib/header/home.svelte
        M	site/src/routes/new/blog/+page.server.ts"
            .to_string();
        let expected = vec![
            Diff::Modified("crates/Cargo.lock".to_string()),
            Diff::Modified("crates/Cargo.toml".to_string()),
            Diff::Modified("crates/api/Cargo.toml".to_string()),
            Diff::Modified("crates/store/Cargo.toml".to_string()),
            Diff::Modified("crates/store/src/lib.rs".to_string()),
            Diff::Deleted("crates/upload/Cargo.toml".to_string()),
            Diff::Deleted("crates/upload/src/lib.rs".to_string()),
            Diff::Deleted("crates/upload/src/uploader.rs".to_string()),
            Diff::Added("crates/uploads/Cargo.toml".to_string()),
            Diff::Renamed(
                "crates/upload/src/error.rs".to_string(),
                "crates/uploads/src/error.rs".to_string(),
            ),
            Diff::Added("crates/uploads/src/git.rs".to_string()),
            Diff::Added("crates/uploads/src/lib.rs".to_string()),
            Diff::Added("crates/uploads/src/new_upload.rs".to_string()),
            Diff::Added("crates/uploads/src/uploader.rs".to_string()),
            Diff::Added("notebook/tasks/timing-bug.md".to_string()),
            Diff::Modified("notebook/wiki/Uploads.md".to_string()),
            Diff::Modified("site/src/lib/header/home.svelte".to_string()),
            Diff::Modified("site/src/routes/new/blog/+page.server.ts".to_string()),
        ];

        let diff = Diff::from_raw(raw);

        for (i, d) in diff.iter().enumerate() {
            assert_eq!(d, &expected[i]);
        }
    }
}
