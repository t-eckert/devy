"use client"

import Link from "next/link"

import useCurrentUser from "@/contexts/useCurrentUser"

export default function UserToken() {
  const currentUser = useCurrentUser()

  if (!currentUser) {
    return (
      <button className="px-2 py-0.5 text-sm font-medium rounded-xl">
        Sign in with GitHub
      </button>
    )
  }

  return (
    <div className="flex flex-row gap-1.5 items-center">
      <Link
        href={`/profiles/${currentUser.username}`}
        className="px-2 py-0.5 text-sm font-medium rounded-xl"
      >
        {currentUser.displayName}
      </Link>
    </div>
  )
}
