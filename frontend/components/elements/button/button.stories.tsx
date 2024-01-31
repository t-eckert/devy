import type { Meta, StoryObj } from "@storybook/react"

import Button from "./button"

const meta = {
	title: "Components/Button",
	component: Button,
} satisfies Meta<typeof Button>

export default meta

type Story = StoryObj<typeof Button>

export const Primary: Story = {
	args: {
		disabled: false,
		children: "Go to settings",
		variant: {
			intent: "primary",
		},
	},
}

export const PrimaryCreate: Story = {
	args: {
		disabled: false,
		children: "Create blog",
		variant: {
			intent: "primary",
			action: "create",
		},
	},
}

export const PrimaryModify: Story = {
	args: {
		disabled: false,
		children: "Pause updates",
		variant: {
			intent: "primary",
			action: "modify",
		},
	},
}

export const PrimaryDestroy: Story = {
	args: {
		disabled: false,
		children: "Delete blog",
		variant: {
			intent: "primary",
			action: "destroy",
		},
	},
}

export const Secondary: Story = {
	args: {
		disabled: false,
		children: "Go to settings",
		variant: {
			intent: "secondary",
		},
	},
}

export const SecondaryCreate: Story = {
	args: {
		disabled: false,
		children: "New comment",
		variant: {
			intent: "secondary",
			action: "create",
		},
	},
}

export const SecondaryModify: Story = {
	args: {
		disabled: false,
		children: "Edit",
		variant: {
			intent: "secondary",
			action: "modify",
		},
	},
}

export const SecondaryDestroy: Story = {
	args: {
		disabled: false,
		children: "Delete comment",
		variant: {
			intent: "secondary",
			action: "destroy",
		},
	},
}

export const Tertiary: Story = {
	args: {
		disabled: false,
		children: "Next",
		variant: {
			intent: "tertiary",
		},
	},
}

export const TertiaryCreate: Story = {
	args: {
		disabled: false,
		children: "Add filter",
		variant: {
			intent: "tertiary",
			action: "create",
		},
	},
}

export const TertiaryModify: Story = {
	args: {
		disabled: false,
		children: "Redo",
		variant: {
			intent: "tertiary",
			action: "modify",
		},
	},
}

export const TertiaryDestroy: Story = {
	args: {
		disabled: false,
		children: "Remove filter",
		variant: {
			intent: "tertiary",
			action: "destroy",
		},
	},
}
