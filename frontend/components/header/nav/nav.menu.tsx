"use client"
import { HamburgerMenuIcon } from "@radix-ui/react-icons"
import Menu, { Item } from "@/components/elements/menu"
import config from "@/config"
import useSession from "@/lib/auth/useSession"
import useStore from "@/lib/useStore"
import type { SessionStore } from "@/lib/auth"

export default function NavMenu() {
  const session = useStore(useSession, (state) => state)
  const navMenuItems: Item[] = [
    {
      type: "link",
      label: "Home",
      href: "/",
    },
    {
      type: "link",
      label: "Changelog",
      tag: `v${config.VERSION}`,
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
  ]

  if (session?.status === "logged-in") {
    navMenuItems.push(...userSignedIn(session))
  } else {
    navMenuItems.push(...userSignedOut())
  }

  return (
    <Menu
      icon={<HamburgerMenuIcon />}
      variant={{ hug: false }}
      ariaLabel="Navigation menu"
      relation="below-aligned-left"
      items={navMenuItems}
    />
  )
}

const userSignedOut = (): Item[] => [
  {
    type: "link",
    label: "Sign in",
    href: "/api/auth/login",
  },
]

const userSignedIn = (session: SessionStore): Item[] => [
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
      session?.clearSession()
    },
  },
]
