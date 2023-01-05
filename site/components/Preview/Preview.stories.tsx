import { ComponentStory, ComponentMeta } from "@storybook/react"
import Preview, { Props } from "./Preview"
import mocks from "./Preview.mocks"

export default {
	title: "components/Preview",
	component: Preview,
	argTypes: {},
} as ComponentMeta<typeof Preview>

const Component: ComponentStory<typeof Preview> = (args) => (
	<Preview {...args} />
)
export const Base = Component.bind({})
Base.args = {
	...mocks.base,
} as Props
