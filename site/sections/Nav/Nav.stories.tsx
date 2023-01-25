import { ComponentStory, ComponentMeta } from "@storybook/react"
import Nav, { Props } from "./Nav"
import mocks from "./Nav.mocks"

export default {
	title: "sections/Nav",
	component: Nav,
	argTypes: {},
} as ComponentMeta<typeof Nav>

const Component: ComponentStory<typeof Nav> = (args) => <Nav {...args} />

export const LoggedOut = Component.bind({})
LoggedOut.args = {
	...mocks.loggedOut,
} as Props

export const LoggedOutOnProfilePage = Component.bind({})
LoggedOutOnProfilePage.args = {
	...mocks.loggedOutOnProfilePage,
} as Props

export const LoggedOutOnArticlePage = Component.bind({})
LoggedOutOnArticlePage.args = {
	...mocks.loggedOutOnArticlePage,
} as Props

export const LoggedOutOnDeepFolderArticlePage = Component.bind({})
LoggedOutOnDeepFolderArticlePage.args = {
	...mocks.loggedOutOnDeepFolderArticlePage,
} as Props

export const LoggedIn = Component.bind({})
LoggedIn.args = {
	...mocks.loggedIn,
} as Props
