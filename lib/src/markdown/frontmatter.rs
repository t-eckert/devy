use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub date: Option<String>,
    pub author: Option<String>,
    pub tags: Option<Vec<String>>,
    pub summary: Option<String>,
    pub is_draft: Option<bool>,
}

pub fn parse_frontmatter(content: &str) -> Option<(Frontmatter, &str)> {
    // Regex to capture the frontmatter section between `---`
    let re = Regex::new(r"(?s)^---\n(.*?)\n---\n(.*)$").unwrap();

    if let Some(captures) = re.captures(content) {
        let frontmatter_str = captures.get(1)?.as_str();
        let body = captures.get(2)?.as_str();

        // Deserialize the YAML frontmatter
        let frontmatter: Frontmatter = serde_yaml::from_str(frontmatter_str).ok()?;

        Some((frontmatter, body))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter() {
        let markdown = r#"---
title: "Introduction to Markdown"
date: "2024-08-31"
author: "John Doe"
tags: ["markdown", "guide", "documentation"]
summary: "A brief introduction to writing markdown documents, including basic syntax and best practices."
is_draft: true
---

# Introduction to Markdown

Markdown is a lightweight markup language..."#;

        let (frontmatter, _) = parse_frontmatter(markdown).unwrap();

        assert_eq!(
            frontmatter.title,
            Some("Introduction to Markdown".to_string())
        );
        assert_eq!(frontmatter.date, Some("2024-08-31".to_string()));
        assert_eq!(frontmatter.author, Some("John Doe".to_string()));
        assert_eq!(
            frontmatter.tags,
            Some(vec![
                "markdown".to_string(),
                "guide".to_string(),
                "documentation".to_string()
            ])
        );
        assert_eq!(frontmatter.summary, Some("A brief introduction to writing markdown documents, including basic syntax and best practices.".to_string()));
        assert_eq!(frontmatter.is_draft, Some(true));
    }
}
