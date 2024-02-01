import type { Meta, StoryObj } from "@storybook/react"

import Uploads from "./uploads"

const meta = {
  title: "Uploads",
  component: Uploads,
} satisfies Meta<typeof Uploads>

export default meta

type Story = StoryObj<typeof Uploads>

export const UploadsStory: Story = {}
