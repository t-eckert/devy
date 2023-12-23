"use client"

import Button from "@/components/button"

import { Dispatch, SetStateAction, useEffect, useState } from "react"
import useSession from "@/lib/auth/useSession"
import useStore from "@/lib/useStore"
import { useQuery } from "@tanstack/react-query"

import api from "@/lib/api"

import CreateLoggedOut from "./create.logged-out"
import { Blog } from "@/models"

interface GitHubRepo {
  id: number
  name: string
  html_url: string
  description: string
  fork: boolean
  language?: string
}

export default function Repos() {
  const session = useStore(useSession, (state) => state)

  let {
    data: repos,
    isLoading,
    refetch,
  } = useQuery({
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

  if (session?.status !== "logged-in") {
    return <CreateLoggedOut />
  }

  return (
    <>
      <p className="mb-4 w-full max-w-md">
        Select one of your existing repositories to create a blog from. Every
        markdown file is converted to a blog post and automatically updated when
        you push changes.
      </p>

      {isLoading ? <Loading /> : <RepoForm repos={repos || []} />}
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
          "p-4 rounded-md text-left border ",
          "flex flex-col items-start gap-1 h-32  hover:shadow-lg transition-all",
          selected === repo
            ? "bg-blue-high dark:bg-neutral-3 border-blue-primary shadow-xl"
            : "border-neutral+1 dark:border-neutral-1 bg-neutral+2 dark:bg-neutral-2 hover:bg-neutral+3 hover:dark:bg-neutral-3",
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

const Loading = () => {
  return (
    <div className="w-full h-44 col-start-1 col-span-full flex flex-col items-center justify-center">
      <span className="font-medium animate-pulse">
        Loading your GitHub repos
      </span>
    </div>
  )
}

const RepoForm = ({ repos }: { repos: GitHubRepo[] }) => {
  const [limit, setLimit] = useState(9)
  const [selected, setSelected] = useState<GitHubRepo | null>(null)

  const onSubmit = async (e: any) => {
    e.preventDefault()
    if (!selected) return
    console.log(e)

    await api.post("/v1/blogs", {
      repoUrl: selected.html_url,
    })

    // if (res.status === 200) {
    //   const blog = (await res.json()) as Blog
    //   window.location.href = `/blogs/${blog.id}`
    // }
  }

  return (
    <form onSubmit={onSubmit} className="flex flex-col gap-4">
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {repos?.length === 0 ? (
          <div className="w-full h-44 col-start-1 col-span-full flex flex-col items-center justify-center">
            <span className="animate-bounce">Loading your GitHub repos</span>
          </div>
        ) : (
          repos
            .slice(0, limit)
            .map((repo: GitHubRepo) => (
              <RepoCard
                key={repo.id}
                repo={repo}
                selected={selected}
                setSelected={setSelected}
              />
            ))
        )}
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
        <Button type="submit" disabled={!selected}>
          Create blog
        </Button>
      </div>
    </form>
  )
}
