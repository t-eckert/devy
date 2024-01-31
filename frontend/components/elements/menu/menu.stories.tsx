import { HamburgerMenuIcon, MoonIcon, SunIcon } from "@radix-ui/react-icons"

import type { Meta, StoryObj } from "@storybook/react"

import Menu from "./menu"

const meta = {
	title: "Elements/Menu",
	component: Menu,
} satisfies Meta<typeof Menu>

const version = "0.5.2"

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
				type: "link",
				label: "Changelog",
				tag: `v${version}`,
				href: "/changelog",
			},
			{
				type: "link",
				label: "About",
				href: "/about",
			},
			{
				type: "link",
				label: "Open an issue",
				href: "https://github.com/t-eckert/devy/issues/new/choose",
			},
			{
				type: "separator",
			},
			{
				type: "link",
				label: "Create your blog",
				href: "/new/blog",
			},
			{
				type: "link",
				label: "Uploads",
				href: "/uploads",
			},
			{
				type: "separator",
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

export const NavMenuLoggedOut: Story = {
	args: {
		icon: <HamburgerMenuIcon className="w-4 aspect-square" />,
		items: [
			{
				type: "link",
				label: "Home",
				href: "/",
			},
			{
				type: "link",
				label: "Changelog",
				tag: `v${version}`,
				href: "/changelog",
			},
			{
				type: "link",
				label: "About",
				href: "/about",
			},
			{
				type: "link",
				label: "Open an issue",
				href: "https://github.com/t-eckert/devy/issues/new/choose",
			},
			{
				type: "separator",
			},
			{
				type: "link",
				label: "Sign in",
				href: "/api/auth/login",
			},
		],
		initialIsOpen: true,
	},
}

export const ThemeToggle: Story = {
	args: {
		icon: <SunIcon className="w-4 aspect-square" />,
		items: [
			{
				type: "button",
				label: "Light",
				onClick: () => {
					console.log("Light")
				},
			},
			{
				type: "button",
				label: "Dark",
				onClick: () => {
					console.log("Dark")
				},
			},
			{
				type: "button",
				label: "System",
				onClick: () => {},
			},
		],
		initialIsOpen: true,
		variant: { hug: true },
	},
}
