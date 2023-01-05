import { ComponentStory, ComponentMeta } from "@storybook/react"
import Link, { Props } from "./Link"
import mocks from "./Link.mocks"

export default {
	title: "components/Link",
	component: Link,
	argTypes: {},
} as ComponentMeta<typeof Link>

const Component: ComponentStory<typeof Link> = (args) => <Link {...args} />

export const Base = Component.bind({})
Base.args = {
	...mocks.base,
} as Props

export const Primary = Component.bind({})
Primary.args = {
	...mocks.primary,
} as Props

export const Secondary = Component.bind({})
Secondary.args = {
	...mocks.secondary,
} as Props

export const Button = Component.bind({})
Button.args = {
	...mocks.button,
} as Props
