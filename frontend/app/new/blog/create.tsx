"use client"

import Card from "@/components/card"
import Button from "@/components/button"
import Json from "@/components/json"

import { useEffect, useState } from "react"
import useSession from "@/lib/auth/useSession"
import useStore from "@/lib/useStore"
import { useQuery } from "@tanstack/react-query"

import CreateLoggedOut from "./create.logged-out"

export default function Repos() {
  const session = useStore(useSession, (state) => state)

  const { data } = useQuery({
    queryKey: ["repos"],
    queryFn: async () => {
      if (session?.status !== "logged-in") return []

      const res = await fetch(
        `https://api.github.com/users/${session.session?.user?.githubUsername}/repos?per_page=100`
      )
      const raw = (await res.json()) as GitHubRepo[]
      return Array.from(raw).filter((repo: GitHubRepo) => !repo.fork)
    },
  })

  const [limit, setLimit] = useState(10)
  const [repos, setRepos] = useState<GitHubRepo[]>(data || [])

  useEffect(() => {
    if (data) {
      setRepos(data.slice(0, limit))
    }
  }, [data, limit])

  if (session?.status !== "logged-in") {
    return <CreateLoggedOut />
  }

  return (
    <>
      <p className="w-full max-w-md">
        Select one of your existing repositories to create a blog from. Every
        markdown file is converted to a blog post and automatically updated when
        you push changes.
      </p>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {data?.map((repo: GitHubRepo) => (
          <Repo key={repo.id} {...repo} />
        ))}
      </div>

      <div className="flex justify-center">
        <Button onClick={() => setLimit(limit + 10)}>Show more</Button>
      </div>

      <div className="border-t border-t-neutral+1 dark:border-t-neutral-1 pt-2 w-full flex justify-end">
        <Button>Create blog</Button>
      </div>
    </>
  )
}

interface GitHubRepo {
  id: number
  name: string
  description: string
  fork: boolean
  language?: string
}

interface RepoProps extends GitHubRepo {}

const Repo = (repo: RepoProps) => (
  <Card className="bg-neutral+3 h-28">
    <div className="p-2">
      <h2 className="font-medium text-neutral-2 dark:text-neutral+2">
        {repo.name}
      </h2>
      <p className="text-sm text-neutral-1 dark:text-neutral+1">
        {repo.description}
      </p>
    </div>
  </Card>
)
