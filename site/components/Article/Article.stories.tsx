import { ComponentStory, ComponentMeta } from "@storybook/react"
import Article, { Props } from "./Article"
import mocks from "./Article.mocks"

export default {
	title: "components/Article",
	component: Article,
	argTypes: {},
} as ComponentMeta<typeof Article>

const Component: ComponentStory<typeof Article> = (args) => (
	<Article {...args} />
)

export const Base = Component.bind({})
Base.args = {
	...mocks.base,
} as Props

export const Gauntlet = Component.bind({})
Gauntlet.args = {
	...mocks.gauntlet,
} as Props

export const RSSPost = Component.bind({})
RSSPost.args = {
	...mocks.rssPost,
} as Props
