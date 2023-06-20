import type { Meta, StoryObj } from "@storybook/react"

import Preview from "./Preview"

const post = {
  slug: "hello-world",
  blog: {
    slug: "blog",
  },
  author: {
    username: "j-doe",
    displayName: "John Doe",
  },
  published: "2021-01-01T00:00:00.000Z",
  title: "Hello World",
  tags: ["hello", "world"],
}

const meta: Meta<typeof Preview> = {
  title: "Components/Models/Post/Preview",
  component: Preview,
  tags: ["autodocs"],
}

export default meta
type Story = StoryObj<typeof Preview>

export const Primary: Story = {
  args: {
    ...post,
  },
}
