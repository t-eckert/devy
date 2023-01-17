import { ComponentStory, ComponentMeta } from "@storybook/react"
import TableOfContents, { Props } from "./TableOfContents"

export default {
    title: "components/TableOfContents",
    component: TableOfContents,
} as ComponentMeta<typeof TableOfContents>

const Component: ComponentStory<typeof TableOfContents> = (args) => <TableOfContents {...args} />

export const Default = Component.bind({})
Default.args = {} as Props