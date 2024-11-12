use super::frontmatter::{parse_frontmatter, Frontmatter};

#[derive(Debug)]
pub struct Markdown {
    pub title: String,
    pub raw: String,
    pub frontmatter: Option<Frontmatter>,
    pub body: String,
    pub headers: Vec<Header>,
}

#[derive(Debug, PartialEq)]
pub struct Header {
    pub level: u8,
    pub text: String,
    pub id: String,
}

pub fn parse_markdown(raw: &str) -> Markdown {
    Markdown::from_str(raw)
}

impl Markdown {
    pub fn is_draft(&self) -> bool {
        if self.frontmatter.is_some() {
            if self.frontmatter.clone().unwrap().is_draft.is_some() {
                return self.frontmatter.clone().unwrap().is_draft.unwrap();
            }
        }
        return false;
    }

    fn from_str(str: &str) -> Markdown {
        let mut headers = Vec::new();
        let mut body = String::from("");

        let (frontmatter, remaining) = match parse_frontmatter(str) {
            Some((frontmatter, remaining)) => (Some(frontmatter), remaining),
            None => (None, str),
        };

        let mut lines = remaining.lines();
        let mut line = lines.next();
        let mut within_code_block = false;
        while line.is_some() {
            if line.unwrap().starts_with("```") {
                within_code_block = !within_code_block;
            }
            if line.unwrap().starts_with("#") && !within_code_block {
                let level = line.unwrap().chars().take_while(|c| *c == '#').count() as u8;
                let text: String = line.unwrap().chars().skip_while(|c| *c == '#').collect();
                let id = text
                    .trim()
                    .chars()
                    .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                    .map(|c| {
                        if c.is_alphanumeric() {
                            c.to_lowercase().to_string()
                        } else {
                            "-".to_string()
                        }
                    })
                    .collect();
                headers.push(Header {
                    level,
                    text: text.trim().to_string(),
                    id,
                });
                // Skip past the de-facto title
                if level == 1 && headers.len() == 1 {
                    line = lines.next()
                }
            }
            body.push_str(line.unwrap());
            body.push_str("\n");
            line = lines.next();
        }

        // Use the frontmatter title if it exists, otherwise use the first header.
        // If there are no headers, default to "Untitled".
        let title;
        if frontmatter.is_some() && frontmatter.clone().unwrap().title.is_some() {
            title = frontmatter.clone().unwrap().title.unwrap();
        } else if headers.first().is_some() {
            title = headers.first().unwrap().text.clone();
        } else {
            title = "Untitled".to_string();
        }

        Markdown {
            title,
            raw: str.to_string(),
            frontmatter,
            body,
            headers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_without_frontmatter() {
        let raw = String::from(
            r#"# Project Title

A brief description of what this project does and who it's for.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Installation

### Prerequisites

- [Node.js](https://nodejs.org/) v14+
- npm v6+ or yarn v1+

### Steps

1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/project-name.git
    ```
2. Navigate to the project directory:
    ```bash
    cd project-name
    ```
3. Install dependencies:
    ```bash
    npm install
    ```

## Usage

To run the project, use the following command:

```bash
npm start
```

### Example Output

Here's what you can expect to see when the project runs:

```text
Server is running on http://localhost:3000
```

## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

### How to Contribute

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Contact

Your Name - [@yourtwitterhandle](https://twitter.com/yourtwitterhandle) - email@example.com

Project Link: [https://github.com/yourusername/project-name](https://github.com/yourusername/project-name)

## Acknowledgments

- [SvelteKit](https://kit.svelte.dev/)
- [Kubernetes](https://kubernetes.io/)
- [OpenAI](https://openai.com/)
"#,
        );
        let body = String::from(
            r#"
A brief description of what this project does and who it's for.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Installation

### Prerequisites

- [Node.js](https://nodejs.org/) v14+
- npm v6+ or yarn v1+

### Steps

1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/project-name.git
    ```
2. Navigate to the project directory:
    ```bash
    cd project-name
    ```
3. Install dependencies:
    ```bash
    npm install
    ```

## Usage

To run the project, use the following command:

```bash
npm start
```

### Example Output

Here's what you can expect to see when the project runs:

```text
Server is running on http://localhost:3000
```

## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

### How to Contribute

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Contact

Your Name - [@yourtwitterhandle](https://twitter.com/yourtwitterhandle) - email@example.com

Project Link: [https://github.com/yourusername/project-name](https://github.com/yourusername/project-name)

## Acknowledgments

- [SvelteKit](https://kit.svelte.dev/)
- [Kubernetes](https://kubernetes.io/)
- [OpenAI](https://openai.com/)
"#,
        );

        let title = String::from("Project Title");
        let frontmatter = None;
        let headers = vec![
            Header {
                level: 1,
                text: "Project Title".to_string(),
                id: "project-title".to_string(),
            },
            Header {
                level: 2,
                text: "Table of Contents".to_string(),
                id: "table-of-contents".to_string(),
            },
            Header {
                level: 2,
                text: "Installation".to_string(),
                id: "installation".to_string(),
            },
            Header {
                level: 3,
                text: "Prerequisites".to_string(),
                id: "prerequisites".to_string(),
            },
            Header {
                level: 3,
                text: "Steps".to_string(),
                id: "steps".to_string(),
            },
            Header {
                level: 2,
                text: "Usage".to_string(),
                id: "usage".to_string(),
            },
            Header {
                level: 3,
                text: "Example Output".to_string(),
                id: "example-output".to_string(),
            },
            Header {
                level: 2,
                text: "Contributing".to_string(),
                id: "contributing".to_string(),
            },
            Header {
                level: 3,
                text: "How to Contribute".to_string(),
                id: "how-to-contribute".to_string(),
            },
            Header {
                level: 2,
                text: "License".to_string(),
                id: "license".to_string(),
            },
            Header {
                level: 2,
                text: "Contact".to_string(),
                id: "contact".to_string(),
            },
            Header {
                level: 2,
                text: "Acknowledgments".to_string(),
                id: "acknowledgments".to_string(),
            },
        ];

        let markdown = Markdown::from_str(&raw);

        assert_eq!(markdown.title, title);
        assert_eq!(markdown.raw, raw);
        assert_eq!(markdown.body, body);
        assert_eq!(markdown.frontmatter, frontmatter);
        assert_eq!(markdown.headers, headers);
    }

    #[test]
    fn text_from_str_with_frontmatter() {
        let raw = String::from(
            r#"---
title: "Introduction to Markdown"
date: "2024-08-31"
author: "John Doe"
tags: ["markdown", "guide", "documentation"]
summary: "A brief introduction to writing markdown documents, including basic syntax and best practices."
---

# Introduction to Markdown

Markdown is a lightweight markup language that you can use to add formatting elements to plaintext text documents. Created by John Gruber in 2004, Markdown is now one of the world's most popular markup languages.

## Why Use Markdown?

Markdown allows you to write using an easy-to-read, easy-to-write plain text format, which then converts to structurally valid HTML. The key benefits include:

- **Simplicity**: It's easy to learn and use.
- **Portability**: Markdown files are just plain text, so they can be opened in any text editor.
- **Flexibility**: Markdown can be converted to various formats, including HTML, PDF, and more.

## Basic Syntax

Here’s a quick rundown of some basic markdown syntax:

### Headings

Use `#` for headings. The number of `#` symbols denotes the heading level:

```markdown
# H1
## H2
### H3
```

### Lists

#### Unordered Lists

Use `-`, `*`, or `+` for unordered lists:

```markdown
- Item 1
- Item 2
- Item 3
```

#### Ordered Lists

Use numbers followed by a period for ordered lists:

```markdown
1. First item
2. Second item
3. Third item
```

### Links

Create a link by wrapping the link text in `[ ]` and the URL in `( )`:

```markdown
[OpenAI](https://openai.com/)
```

### Images

Add an image by using `![alt text](image_url)`:

```markdown
![Markdown Logo](https://markdown-here.com/img/icon256.png)
```

### Code Blocks

Use triple backticks for code blocks:

<pre>
```javascript
console.log("Hello, World!");
```
</pre>

### Inline Code

Use backticks for inline code:

```markdown
This is `inline code`.
```

## Conclusion

Markdown is a versatile tool that simplifies the process of creating formatted documents. Whether you're writing for a blog, creating documentation, or drafting emails, Markdown's syntax is both intuitive and powerful.

---

Written by [John Doe](https://johndoe.com). For more guides, check out my [blog](https://johndoe.com/blog).
"#,
        );

        let title = String::from("Introduction to Markdown");
        let body = String::from(
            r#"

Markdown is a lightweight markup language that you can use to add formatting elements to plaintext text documents. Created by John Gruber in 2004, Markdown is now one of the world's most popular markup languages.

## Why Use Markdown?

Markdown allows you to write using an easy-to-read, easy-to-write plain text format, which then converts to structurally valid HTML. The key benefits include:

- **Simplicity**: It's easy to learn and use.
- **Portability**: Markdown files are just plain text, so they can be opened in any text editor.
- **Flexibility**: Markdown can be converted to various formats, including HTML, PDF, and more.

## Basic Syntax

Here’s a quick rundown of some basic markdown syntax:

### Headings

Use `#` for headings. The number of `#` symbols denotes the heading level:

```markdown
# H1
## H2
### H3
```

### Lists

#### Unordered Lists

Use `-`, `*`, or `+` for unordered lists:

```markdown
- Item 1
- Item 2
- Item 3
```

#### Ordered Lists

Use numbers followed by a period for ordered lists:

```markdown
1. First item
2. Second item
3. Third item
```

### Links

Create a link by wrapping the link text in `[ ]` and the URL in `( )`:

```markdown
[OpenAI](https://openai.com/)
```

### Images

Add an image by using `![alt text](image_url)`:

```markdown
![Markdown Logo](https://markdown-here.com/img/icon256.png)
```

### Code Blocks

Use triple backticks for code blocks:

<pre>
```javascript
console.log("Hello, World!");
```
</pre>

### Inline Code

Use backticks for inline code:

```markdown
This is `inline code`.
```

## Conclusion

Markdown is a versatile tool that simplifies the process of creating formatted documents. Whether you're writing for a blog, creating documentation, or drafting emails, Markdown's syntax is both intuitive and powerful.

---

Written by [John Doe](https://johndoe.com). For more guides, check out my [blog](https://johndoe.com/blog).
"#,
        );
        let frontmatter = Frontmatter {
            title: Some("Introduction to Markdown".to_string()),
            date: Some("2024-08-31".to_string()),
            author: Some("John Doe".to_string()),
            tags: Some(vec!["markdown".to_string(), "guide".to_string(), "documentation".to_string()]),
            summary:Some("A brief introduction to writing markdown documents, including basic syntax and best practices.".to_string()),
            is_draft: None,
        };

        let headers = vec![
            Header {
                level: 1,
                text: "Introduction to Markdown".to_string(),
                id: "introduction-to-markdown".to_string(),
            },
            Header {
                level: 2,
                text: "Why Use Markdown?".to_string(),
                id: "why-use-markdown".to_string(),
            },
            Header {
                level: 2,
                text: "Basic Syntax".to_string(),
                id: "basic-syntax".to_string(),
            },
            Header {
                level: 3,
                text: "Headings".to_string(),
                id: "headings".to_string(),
            },
            Header {
                level: 3,
                text: "Lists".to_string(),
                id: "lists".to_string(),
            },
            Header {
                level: 4,
                text: "Unordered Lists".to_string(),
                id: "unordered-lists".to_string(),
            },
            Header {
                level: 4,
                text: "Ordered Lists".to_string(),
                id: "ordered-lists".to_string(),
            },
            Header {
                level: 3,
                text: "Links".to_string(),
                id: "links".to_string(),
            },
            Header {
                level: 3,
                text: "Images".to_string(),
                id: "images".to_string(),
            },
            Header {
                level: 3,
                text: "Code Blocks".to_string(),
                id: "code-blocks".to_string(),
            },
            Header {
                level: 3,
                text: "Inline Code".to_string(),
                id: "inline-code".to_string(),
            },
            Header {
                level: 2,
                text: "Conclusion".to_string(),
                id: "conclusion".to_string(),
            },
        ];

        let markdown = Markdown::from_str(&raw);

        assert_eq!(markdown.title, title);
        assert_eq!(markdown.raw, raw);
        assert_eq!(markdown.body, body);
        assert_eq!(markdown.frontmatter, Some(frontmatter));
        assert_eq!(markdown.headers, headers);
    }
}
