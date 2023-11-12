"use client"

import { GitHubLogoIcon, HamburgerMenuIcon } from "@radix-ui/react-icons"

import Link from "@/components/link"
import Avatar from "@/components/avatar"

import Menu, { Item } from "@/components/menu"
import useSession from "@/lib/auth/useSession"
import type Session from "@/lib/auth/Session"
import useStore from "@/lib/useStore"
import config from "@/config"

export default function Nav() {
  const session = useStore(useSession, (state) => state)

  const items: Item[] = [
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

  return (
    <nav className="flex flex-row gap-1 items-center">
      {session?.session ? <Token session={session.session} /> : <Login />}
      <Menu
        icon={<HamburgerMenuIcon />}
        ariaLabel="Navigation menu"
        relation="below-aligned-left"
        items={items}
      />
    </nav>
  )
}

const Login = () => (
  <Link
    href="/api/auth/login"
    prefetch={false}
    variant={{ underline: false }}
    className=""
  >
    <div className="pl-2 pr-2 py-0.5 flex flex-row rounded-l-full gap-2 items-center transition-all">
      <GitHubLogoIcon className="w-4 h-4" />
      <span className="text-sm">Sign in</span>
    </div>
  </Link>
)

const Token = ({ session }: { session: Session }) => (
  <Link
    href={`/profiles/${session.user.username}`}
    variant={{ underline: false }}
    className="rounded-xl dark:text-neutral+2 dark:hover:text-neutral+3 dark:hover:bg-neutral-1"
  >
    <div className="pl-1 pr-2 py-0.5 flex flex-row rounded-l-full gap-2 items-center transition-all">
      <Avatar
        name={session.profile.displayName}
        avatarUrl={session.profile.avatarUrl}
      />
      <span className="text-sm">{session.profile.displayName}</span>
    </div>
  </Link>
)
