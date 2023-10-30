"use client"

import { GitHubLogoIcon } from "@radix-ui/react-icons"

import Link from "@/components/link"
import Avatar from "@/components/avatar"

import Menu from "@/lib/nav/Menu"
import useSession from "@/lib/auth/useSession"
import type Session from "@/lib/auth/Session"
import useStore from "@/lib/useStore"

export default function Nav() {
  const session = useStore(useSession, (state) => state.session)

  return (
    <nav className="flex flex-row gap-2">
      {session ? <Token session={session} /> : <Login />}
      <Menu />
    </nav>
  )
}

const Login = () => (
  <Link href="/api/auth/login" prefetch={false}>
    <div className="pl-2 pr-2 py-0.5 flex flex-row rounded-l-full gap-2 items-center transition-all">
      <GitHubLogoIcon className="w-4 h-4" />
      <span className="text-xs font-medium">Sign in</span>
    </div>
  </Link>
)

const Token = ({ session }: { session: Session }) => (
  <Link href={`/profiles/${session.user.username}`}>
    <div className="pl-2 pr-2 py-0.5 flex flex-row rounded-l-full gap-2 items-center transition-all">
      <Avatar
        name={session.profile.displayName}
        avatarUrl={session.profile.avatarUrl}
      />
      <span className="text-sm font-medium">{session.profile.displayName}</span>
    </div>
  </Link>
)