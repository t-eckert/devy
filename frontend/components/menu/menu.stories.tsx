import { HamburgerMenuIcon } from "@radix-ui/react-icons"

import type { Meta, StoryObj } from "@storybook/react"

import Menu from "./menu"

const meta = {
	title: "Components/Menu",
	component: Menu,
} satisfies Meta<typeof Menu>

export default meta

type Story = StoryObj<typeof Menu>

export const NavMenuLoggedIn: Story = {
	args: {
		icon: <HamburgerMenuIcon className="w-4 aspect-square" />,
		items: [
			{
				type: "link",
				label: "Home",
				href: "/",
			},
			{
				type: "separator",
			},
			{
				type: "link",
				label: "Following",
				tag: "200",
				href: "/following",
			},
			{
				type: "link",
				label: "Following",
				tag: "2.3k",
				href: "/following",
			},
			{
				type: "link",
				label: "Uploads",
				href: "/uploads",
			},
			{
				type: "link",
				label: "Settings",
				href: "/settings",
			},
			{
				type: "separator",
			},
			{
				type: "link",
				label: "Changelog",
				tag: "v0.5.2",
				href: "/changelog",
			},
			{
				type: "button",
				label: "Sign out",
				onClick: () => {
					console.log("Sign out")
				},
			},
		],
		initialIsOpen: true,
	},
}
