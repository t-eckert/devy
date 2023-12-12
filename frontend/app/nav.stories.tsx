
import type { Meta, StoryObj } from "@storybook/react"

import Nav from "./nav"

const meta = {
  title: "Lockups/Nav",
  component: Nav,
} satisfies Meta<typeof Nav>


export default meta

type Story = StoryObj<typeof Nav>

export const Default: Story = {
  args: {
  },
}
