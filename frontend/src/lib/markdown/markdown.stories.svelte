<script context="module">
	import Markdown from "./markdown.svelte"

	export const meta = {
		title: "markdown/Markdown",
		component: Markdown
	}
</script>

<script lang="ts">
	import { Template, Story } from "@storybook/addon-svelte-csf"

	let articleSource = `# The Power of Markdown: A Comprehensive Guide

Markdown is a lightweight markup language that you can use to add formatting elements to plaintext text documents. Created by John Gruber in 2004, Markdown is now one of the world's most popular markup languages.

## Table of Contents
1. [Introduction](#introduction)
2. [Basic Formatting](#basic-formatting)
3. [Lists](#lists)
4. [Links and Images](#links-and-images)
5. [Code Blocks](#code-blocks)
6. [Blockquotes](#blockquotes)
7. [Tables](#tables)
8. [Conclusion](#conclusion)

## Introduction

Markdown allows you to write using an easy-to-read, easy-to-write plain text format, which then converts to structurally valid HTML.

## Basic Formatting

Here are some of the most common formatting options:

- **Bold**: \`**bold text**\` or \`__bold text__\`
- *Italic*: \`*italicized text*\` or \`_italicized text_\`
- ~~Strikethrough~~: \`~~strikethrough text~~\`

Example:

\`\`\`markdown
**This is bold text**

*This is italicized text*

~~This is strikethrough text~~
\`\`\`

## Lists

### Unordered List

- Item 1
- Item 2
- Item 3

### Ordered List

1. First item
2. Second item
3. Third item

### Nested List

- Item 1
  - Subitem 1
  - Subitem 2
- Item 2

## Links and Images

You can create [links](https://www.example.com) and add images.

\`\`\`markdown
[This is a link](https://www.example.com)

![This is an image](https://www.example.com/image.jpg)
\`\`\`

## Code Blocks

Markdown supports inline code as well as multiple code blocks with different languages.

### Inline Code

Use backticks for \`inline code\`.

### Code Block in Python

\`\`\`python
def hello_world():
    print("Hello, world!")
\`\`\`

### Code Block in JavaScript

\`\`\`javascript
function helloWorld() {
    console.log("Hello, world!");
}
\`\`\`

## Blockquotes

> Markdown is a lightweight markup language with plain text formatting syntax.
>
> — John Gruber

## Tables

You can create tables using the following syntax:

\`\`\`markdown
| Syntax      | Description |
| ----------- | ----------- |
| Header      | Title       |
| Paragraph   | Text        |
\`\`\`

Result:

| Syntax      | Description |
| ----------- | ----------- |
| Header      | Title       |
| Paragraph   | Text        |

## Conclusion

Markdown is a versatile and easy-to-use language that can handle a variety of formatting needs. Whether you're a developer looking to document your code, a writer crafting articles, or just someone looking to make your plain text a little more dynamic, Markdown is a tool worth exploring.

Happy writing!
`
</script>

<Template let:args>
	<div class="w-full max-w-3xl mx-auto">
		<Markdown {...args} />
	</div>
</Template>

<Story name="Article" args={{ source: articleSource }}></Story>
