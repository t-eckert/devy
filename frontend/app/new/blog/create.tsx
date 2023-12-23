"use client"

import Button from "@/components/button"

import { Dispatch, SetStateAction, useEffect, useState } from "react"
import useSession from "@/lib/auth/useSession"
import useStore from "@/lib/useStore"
import { useQuery } from "@tanstack/react-query"

import api from "@/lib/api"

import CreateLoggedOut from "./create.logged-out"
import { Blog } from "@/models"
import Input from "@/components/input"
import { Session } from "@/lib/auth"
import Card from "@/components/card"

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

  if (session?.status !== "logged-in" || session.session === null) {
    return <CreateLoggedOut />
  }

  return (
    <>
      {isLoading ? (
        <Loading />
      ) : (
        <RepoForm repos={repos || []} session={session.session} />
      )}
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

const RepoForm = ({
  repos,
  session,
}: {
  repos: GitHubRepo[]
  session: Session
}) => {
  const [limit, setLimit] = useState(9)
  const [selected, setSelected] = useState<GitHubRepo | null>(null)
  const [blogName, setBlogName] = useState<string>("")
  const [isSubmittable, setIsSubmittable] = useState<boolean>(false)
  const [createButtonText, setCreateButtonText] = useState<string>("")

  useEffect(() => {
    setIsSubmittable(blogName.length > 0 && selected !== null)
    if (isSubmittable) {
      setCreateButtonText(`Create "${blogName}" using ${selected?.name}`)
    } else {
      setCreateButtonText("You still need something else")
    }
  }, [blogName, selected])

  const onSubmit = async (e: any) => {
    e.preventDefault()
    if (!selected) return
    console.log(e)

    await api.post("/v1/blogs", {
      username: session.user.username,
      name: selected.name,
      repoUrl: selected.html_url,
      githubId: selected.id,
      githubName: selected.name,
      metadata: selected,
    })

    // if (res.status === 200) {
    //   const blog = (await res.json()) as Blog
    //   window.location.href = `/blogs/${blog.id}`
    // }
  }

  return (
    <form className="grid grid-cols-4 gap-y-8 gap-x-4">
      <div className="col-span-1">
        <span className="text-xs font-medium text-neutral">Step 1</span>
        <h2 className="font-medium text-neutral+3">Name your blog</h2>
        <p className="text-sm text-neutral+1">
          Pick a display name for your blog. This can be anything you want and
          you can change it later.
        </p>
      </div>
      <div className="p-3 col-span-3">
        <Input
          className="w-96"
          label="Blog name"
          value={blogName}
          setValue={setBlogName}
          placeholder={`${session.profile.displayName}'s Blog`}
        />
      </div>
      <div>
        <span className="text-xs font-medium text-neutral">Step 2</span>
        <h2 className="font-medium text-neutral+3">Pick a repository</h2>
        <p className="text-sm text-neutral+1">
          Select one of your GitHub repositories to publish posts from. All
          markdown docs in the repository will be converted into blog posts and
          updated on pushes to the primary branch.
        </p>
      </div>
      <div className="p-3 col-span-3">
        <div className="flex flex-col gap-3">
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {repos.slice(0, limit).map((repo: GitHubRepo) => (
              <RepoCard
                key={repo.id}
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
        </div>
      </div>

      <div className="col-span-4 border-b border-b-neutral+1 dark:border-b-neutral-1" />
      <div className="col-start-2 pl-3">
        <Button onSubmit={onSubmit} disabled={!isSubmittable}>
          Create your blog
        </Button>
      </div>
    </form>
  )
}
