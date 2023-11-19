import type { Meta, StoryObj } from "@storybook/react"

import Markdown from "./markdown"

const meta = {
	title: "Components/Markdown",
	component: Markdown,
} satisfies Meta<typeof Markdown>

export default meta

type Story = StoryObj<typeof Markdown>

export const Sampling: Story = {
	args: {
		content: `
# Heading 1
## Heading 2
### Heading 3
#### Heading 4
##### Heading 5
###### Heading 6

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras a justo iaculis elit tristique varius. Proin eu arcu eu mauris vehicula sollicitudin eu eget leo. Sed in tortor rhoncus arcu euismod mattis id ut enim. Vivamus vitae placerat justo. Nulla facilisi. Aenean sed lectus sapien. Sed vitae sollicitudin erat. Aenean lectus tortor, iaculis eget rutrum nec, pulvinar nec dolor. Phasellus orci dui, facilisis quis dolor quis, dapibus tempor velit. Nulla egestas hendrerit mauris at auctor. Quisque volutpat scelerisque sem quis mattis. Nam sagittis, dui varius vehicula rhoncus, odio felis lobortis odio, in semper orci magna sollicitudin est. Nulla laoreet id lacus id facilisis.

Sed non lacinia purus. Vestibulum hendrerit metus vel odio pulvinar fringilla. Sed imperdiet venenatis sem ut hendrerit. Curabitur tincidunt justo augue, non ornare tellus hendrerit vel. Vestibulum ultricies massa vitae iaculis efficitur. Vestibulum sit amet turpis ut ante placerat iaculis elementum vitae turpis. Nullam at auctor nisl.

Nunc volutpat nunc ac ipsum euismod sollicitudin. Praesent tristique massa leo, et dapibus tortor gravida eu. Proin in pulvinar nisl, a sagittis tellus. Integer at lobortis diam, sit amet maximus tellus. In hac habitasse platea dictumst. Maecenas ac velit ultricies, hendrerit purus sit amet, iaculis libero. Duis sollicitudin sem id velit ullamcorper, vel malesuada mi sodales. Integer nec ligula ligula. Vivamus et sapien magna. Nunc venenatis ipsum odio, congue ornare ligula finibus vel.

- Item 
- Item
- Item

1. Item
2. Item
3. Item

> This is a blockquote.
> 
> This is the second paragraph in the blockquote.


| Syntax      | Description |
| :---------- | :---------- |
| Header      | Title       |
| Paragraph   | Text        |

**This is bold text**

*This text is italicized*

~~This was mistaken text~~

**This text is _extremely_ important**

***All this text is important***

![This is a alt text.](https://images.unsplash.com/photo-1673201712131-64e971d8a7dc?q=80&w=2574&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D "This is a sample image.")

[This is a link](https://www.google.com)

\`\`\`javascript
// This is a comment
var x = 5; // This is another comment
\`\`\`

\`\`\`python
# This is a comment
x = 5
print(x)
\`\`\`
	`,
	},
}

export const ComputingMachineryAndIntelligence: Story = {
	args: {
		content: `
# Summary of Alan Turing's "Computing Machinery and Intelligence" (1950)

Alan Turing's 1950 paper "Computing Machinery and Intelligence" is a seminal work in the field of artificial intelligence and is primarily focused on the question of whether machines can exhibit human-like intelligence. Here is a summary of the key points and ideas from the paper:

1. **The Imitation Game**: Turing introduces the concept of the "imitation game," which is now commonly known as the Turing Test. In this test, a human judge engages in a conversation with both a human and a machine (hidden from view), and if the judge cannot reliably distinguish between the two based on their responses, then the machine is said to have passed the test and demonstrated intelligence.

2. **The Turing Test as a Defining Criterion**: Turing argues that the question of whether a machine can think is too abstract and unanswerable. Instead, he proposes that we should focus on whether a machine can mimic human intelligence well enough to pass the imitation game.

3. **Objections and Responses**: Turing anticipates and addresses several objections to his proposal, such as the argument that the test is too limited or that it merely tests the ability to deceive. He argues that these objections do not undermine the fundamental idea that the imitation game provides a practical and meaningful way to assess machine intelligence.

4. **Computing Machinery and the Brain**: Turing briefly discusses the analogy between the operations of a digital computer and the processes of human thought. He suggests that both can be thought of as manipulating symbols according to specific rules.

5. **Can Machines Think?**: Turing acknowledges that his paper does not definitively answer the question of whether machines can think. He asserts that the practicality of the imitation game as a test for intelligence is more important than settling the philosophical question of whether machines possess consciousness or true understanding.

6. **Learning and Improvement**: Turing speculates about the potential for machines to learn and improve over time. He envisions the possibility of machines that can adapt and develop their abilities, further blurring the line between human and machine intelligence.

In summary, Alan Turing's 1950 paper "Computing Machinery and Intelligence" is a foundational work that introduced the concept of the Turing Test as a way to assess machine intelligence. It shifted the focus of discussions about artificial intelligence from abstract philosophical questions to practical and testable criteria for machine behavior.
`,
	},
}

export const Code: Story = {
	args: {
		content: `
# Code

\`\`\`javascript
// This is a comment
var x = 5; // This is another comment
\`\`\`

\`\`\`python
# This is a comment
x = 5
print(x)
\`\`\`
`,
	},
}
