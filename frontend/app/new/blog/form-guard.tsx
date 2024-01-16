"use client"

import useSession from "@/lib/auth/useSession"
import useStore from "@/lib/useStore"

import Form from "./form"
import useUserRepos from "./useUserRepos"

export default function FormGuard() {
  const sessionStore = useStore(useSession, (state) => state)
  const { data: repos } = useUserRepos("t-eckert")

  if (!repos || !sessionStore) return null

  return (
    <div>
      <Form
        username={sessionStore.session?.user.username || ""}
        repos={repos}
      />
    </div>
  )
}
