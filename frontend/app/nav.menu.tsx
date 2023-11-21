"use client"
import {
  GitHubLogoIcon,
  HamburgerMenuIcon,
  MoonIcon,
  SunIcon,
} from "@radix-ui/react-icons"
import Menu, { Item } from "@/components/menu"
import config from "@/config"
import useSession from "@/lib/auth/useSession"
import type Session from "@/lib/auth/Session"
import useStore from "@/lib/useStore"

export default function NavMenu() {

  const session = useStore(useSession, (state) => state)
  const navMenuItems: Item[] = [
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
      label: "Followers",
      tag: "2.3k",
      href: "/followers",
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
      tag: `v${config.VERSION}`,
      href: "/changelog",
    },
    {
      type: "button",
      label: "Sign out",
      onClick: () => {
        session?.clearSession()
      },
    },
  ]


  return <Menu
    icon={<HamburgerMenuIcon />}
    variant={{ hug: false }}
    ariaLabel="Navigation menu"
    relation="below-aligned-left"
    items={navMenuItems}
  />
}
