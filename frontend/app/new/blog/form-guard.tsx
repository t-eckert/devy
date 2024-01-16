"use client"

import useSession from "@/lib/auth/useSession"
import useStore from "@/lib/useStore"

import Form from "./form"
import useUserRepos from "./useUserRepos"

export default function FormGuard() {
  const sessionStore = useStore(useSession, (state) => state)
  const { data: repos, status: repoStatus } = useUserRepos(
    sessionStore?.session?.user.username
  )

  if (!sessionStore) return null

  const status = sessionStore.status
  if (status === "logged-out") return <p>Log in to create a new blog.</p>

  if (repoStatus === "pending" || !repos) return <p>Loading...</p>

  return (
    <Form username={sessionStore.session?.user.username || ""} repos={repos} />
  )
}
