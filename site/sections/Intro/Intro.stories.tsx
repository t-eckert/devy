import { ComponentStory, ComponentMeta } from "@storybook/react"
import Intro from "./Intro"

export default {
  title: "sections/Intro",
  component: Intro,
  argTypes: {},
} as ComponentMeta<typeof Intro>

const Component: ComponentStory<typeof Intro> = () => <Intro />

export const Base = Component.bind({})

