import { ComponentStory, ComponentMeta } from "@storybook/react"
import Byline, { Props } from "./Byline"
import fixtures from "fixtures/post"

export default {
  title: "components/Byline",
  component: Byline,
  argTypes: {},
} as ComponentMeta<typeof Byline>

const Component: ComponentStory<typeof Byline> = (args) => (
  <Byline {...args} />
)

export const Base = Component.bind({})
Base.args = {
  postMetadata: fixtures.postsMetadata[0],
} as Props
