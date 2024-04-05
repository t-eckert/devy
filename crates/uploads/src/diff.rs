#[derive(Debug, PartialEq)]
pub enum Diff {
    Added(String),
    Deleted(String),
    Modified(String),
    Renamed(i32, String, String),
}

pub fn diff_files(raw: String) -> Vec<Diff> {
    let mut diffs = Vec::new();
    for line in raw.lines() {
        let mut parts = line.split('\t');
        let status = parts.next().unwrap();
        let (from, to): (&str, &str) = match status {
            "A" => {
                let to = parts.next().unwrap();
                diffs.push(Diff::Added(to.to_string()));
                continue;
            }
            "D" => {
                let from = parts.next().unwrap();
                diffs.push(Diff::Deleted(from.to_string()));
                continue;
            }
            "M" => {
                let to = parts.next().unwrap();
                diffs.push(Diff::Modified(to.to_string()));
                continue;
            }
            "R" => {
                let from = parts.next().unwrap();
                let to = parts.next().unwrap();
                let similarity = parts.next().unwrap();
                diffs.push(Diff::Renamed(
                    similarity.parse().unwrap(),
                    from.to_string(),
                    to.to_string(),
                ));
                continue;
            }
            _ => continue,
        };
    }
    diffs
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
                71,
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

        let diff = diff_files(raw);

        for (i, d) in diff.iter().enumerate() {
            assert_eq!(d, &expected[i]);
        }
    }
}
