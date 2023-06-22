import type { Meta, StoryObj } from "@storybook/react"

import Preview from "./Preview"
import posts from "@/fixtures/posts"

const post = posts[0]

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
