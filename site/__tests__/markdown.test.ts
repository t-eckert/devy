import parse, { headers } from "markdown"
import type { Header } from "markdown"

it("parses markdown to html", () => {
  const given = `# Hello`
  const expected = `<h1 id="hello">Hello</h1>
`

  const actual = parse(given)

  expect(actual).toEqual(expected)
})

it("gets headers from markdown", () => {
  const given = `# Hello

## Second header

Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim 
labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet.
Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem
est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud 
officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat 
reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia 
voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt 
duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt 
velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est 
culpa et culpa duis.

### Animals

### Peanuts

## But wait there's more!

  ## this one shouldn't actually get caught.

### This one should
`
  const expected = [
    {
      id: "hello",
      value: "Hello",
      level: 1
    },
    {
      id: "second-header",
      level: 2,
      value: "Second header",
    },
    {
      id: "animals",
      level: 3,
      value: "Animals",
    },
    {
      id: "peanuts",
      level: 3,
      value: "Peanuts",
    },
    {
      id: "but-wait-theres-more",
      level: 2,
      value: "But wait there's more!",
    },
    {
      id: "this-one-should",
      level: 3,
      value: "This one should",
    },
  ]

  const actual = headers(given)

  expect(actual).toEqual(expected)
})
