"use client"

import Card from "@/components/card"
import Button from "@/components/button"
import Json from "@/components/json"

import { Dispatch, SetStateAction, useEffect, useState } from "react"
import useSession from "@/lib/auth/useSession"
import useStore from "@/lib/useStore"
import { useQuery } from "@tanstack/react-query"

import CreateLoggedOut from "./create.logged-out"

interface GitHubRepo {
  id: number
  name: string
  description: string
  fork: boolean
  language?: string
}

export default function Repos() {
  const session = useStore(useSession, (state) => state)

  const { data: repos, refetch } = useQuery({
    queryKey: ["repos"],
    queryFn: async () => {
      if (session?.status !== "logged-in") return []

      const res = await fetch(
        `https://api.github.com/users/${session.session?.user?.githubUsername}/repos?per_page=100&sort=updated`
      )
      const raw = (await res.json()) as GitHubRepo[]
      return Array.from(raw).filter((repo: GitHubRepo) => !repo.fork)
    },
  })

  useEffect(() => {
    refetch()
  }, [session])

  const [limit, setLimit] = useState(9)
  const [selected, setSelected] = useState<GitHubRepo | null>(null)

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

      <form className="flex flex-col gap-4">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {repos?.slice(0, limit).map((repo: GitHubRepo) => (
            <RepoCard
              repo={repo}
              selected={selected}
              setSelected={setSelected}
            />
          ))}
        </div>

        <div className="flex justify-center">
          {limit < 0 ? (
            <Button
              onClick={() => {
                setLimit(9)
              }}
              variant={{ intent: "secondary" }}
              type="button"
            >
              Show fewer
            </Button>
          ) : (
            <Button
              onClick={() => {
                setLimit(-1)
              }}
              variant={{ intent: "secondary" }}
              type="button"
            >
              Show all
            </Button>
          )}
        </div>

        <div className="border-t border-t-neutral+1 dark:border-t-neutral-1 pt-2 w-full flex justify-end">
          <Button
            onClick={() => alert(JSON.stringify(selected))}
            disabled={!selected}
          >
            Create blog
          </Button>
        </div>
      </form>
    </>
  )
}

const RepoCard = ({
  repo,
  selected,
  setSelected,
}: {
  repo: GitHubRepo
  selected: GitHubRepo | null
  setSelected: Dispatch<SetStateAction<GitHubRepo | null>>
}) => {
  return (
    <button
      onClick={() => {
        if (selected === repo) {
          setSelected(null)
        } else {
          setSelected(repo)
        }
      }}
      type="button"
      key={repo.id}
    >
      <div
        className={[
          "p-4 rounded-md text-left border border-neutral+1 dark:border-neutral-1 bg-neutral+2 dark:bg-neutral-2",
          "flex flex-col items-start gap-1 h-32 hover:bg-neutral+3 hover:dark:bg-neutral-3 hover:shadow-lg transition-all",
          selected === repo ? "bg-neutral+3 dark:bg-neutral-3 shadow-xl" : "",
        ].join(" ")}
      >
        <h2 className="font-medium text-neutral-2 dark:text-neutral+2">
          {repo.name}
        </h2>
        <p className="text-sm text-neutral-1 dark:text-neutral+1">
          {repo.description}
        </p>
      </div>
    </button>
  )
}
