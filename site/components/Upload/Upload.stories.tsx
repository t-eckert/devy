import { ComponentStory, ComponentMeta } from "@storybook/react"
import Upload, { Props } from "./Upload"
import mocks from "./Upload.mocks"

export default {
	title: "components/Upload",
	component: Upload,
} as ComponentMeta<typeof Upload>

const Component: ComponentStory<typeof Upload> = (args) => <Upload {...args} />
export const Base = Component.bind({})
Base.args = {
	...mocks.base,
} as Props
