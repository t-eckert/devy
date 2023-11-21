"use client"

import {
  GitHubLogoIcon,
} from "@radix-ui/react-icons"

import Link from "@/components/link"
import Avatar from "@/components/avatar"

import useStore from "@/lib/useStore"
import useSession from "@/lib/auth/useSession"
import type Session from "@/lib/auth/Session"

export default function NavToken() {
  const session = useStore(useSession, (state) => state)

  if (session?.session) {
    return <Token session={session.session} />
  }

  return <Login />
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
