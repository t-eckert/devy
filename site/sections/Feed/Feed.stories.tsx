import { ComponentStory, ComponentMeta } from "@storybook/react"
import Feed, { Props } from "./Feed"
import mocks from "./Feed.mocks"

export default {
	title: "sections/Feed",
	component: Feed,
	argTypes: {},
} as ComponentMeta<typeof Feed>

const Component: ComponentStory<typeof Feed> = (args) => <Feed {...args} />

export const PopularFeed = Component.bind({})
PopularFeed.args = {
	...mocks.popularFeed,
} as Props
