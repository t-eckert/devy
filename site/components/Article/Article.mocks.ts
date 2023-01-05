import { Props } from "./Article"
import parse from "markdown"

const base: Props = {
	html: `<h1>Hello World</h1>
<p>
	Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur
	mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem 
	est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure
	elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia 
	voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua 
	reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea 
	consectetur et est culpa et culpa duis.
</p>`,
}

const gauntlet: Props = {
	html: parse(`# Heading 1
## Heading 2
### Heading 3
#### Heading 4
##### Heading 5
###### Heading 6

Emphasis:

*Italic*

_Italic_

**Bold**

__Bold__

Combined emphasis:

**_Bold and italic_**

Lists:

1. First item
2. Second item
3. Third item

* First item
* Second item
* Third item

- First item
- Second item
- Third item

Links:

[Link text](https://www.example.com)

[Link text with title](https://www.example.com "Link title")

Images:

![Alt text](https://www.example.com/image.jpg)

![Alt text with title](https://www.example.com/image.jpg "Image title")

Code:

`),
}

const rssPost: Props = {
	html: parse(`# The Benefits of RSS

RSS (Really Simple Syndication) is a web feed that allows users to access updates to websites in a standardized, computer-readable format. These updates can include blog posts, news articles, and other types of content.

There are several benefits to using RSS:

## 1. Time-saving

RSS allows you to quickly and easily stay updated with your favorite websites without having to visit each one individually. This can save you a significant amount of time, especially if you follow a large number of websites.

## 2. Customization

With RSS, you can choose which updates you receive and how you receive them. You can use an RSS reader to organize and prioritize your feeds, or you can opt to receive updates via email.

## 3. Ad-free

Many RSS readers allow you to view updates in a stripped-down, ad-free format. This can make for a more pleasant reading experience and can also help to reduce clutter and distractions.

## 4. Accessibility

RSS allows you to access updates from your favorite websites from any device with an internet connection. This can be especially useful when you're on the go and don't have access to a particular device or browser.

Overall, RSS is a convenient and efficient way to stay up-to-date with your favorite websites and content. Give it a try and see how it can help you stay organized and informed.`),
}

const mocks = {
	base,
	gauntlet,
	rssPost,
}

export default mocks
