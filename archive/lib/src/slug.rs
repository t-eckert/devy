use slugify::slugify;

/// Slugify a string.
/// This is just a wrapper around the `slugify` crate.
pub fn slug(s: &str) -> String {
    slugify!(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slug() {
        let cases = vec![
            ("Hello, World!", "hello-world"),
            ("Hello, 世界!", "hello-shi-jie"),
        ];

        for (input, expected) in cases {
            assert_eq!(slug(input), expected);
        }
    }
}
