import { ComponentStory, ComponentMeta } from "@storybook/react"
import Article, { Props } from "./Article"
import mocks from "./Article.mocks"

export default {
	title: "Article",
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
