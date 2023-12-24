"use client"

import { useState } from "react"

import Button from "@/components/button"

import useCreateState from "./useCreateState"

interface GitHubRepo {
  id: number
  name: string
  html_url: string
  description: string
  fork: boolean
  language?: string
}

export default function CreateFormStep2() {
  const { repos, selectedRepo, setSelectedRepo } = useCreateState()
  const [limit, setLimit] = useState(9)

  return (
    <div>
      <span className="text-xs font-medium text-neutral">Step 2</span>

      <div className="flex flex-col sm:flex-row gap-4 sm:gap-8">
        <div className="w-96 sm:w-60">
          <h2 className="font-medium text-neutral-3 dark:text-neutral+3">
            Pick a repository
          </h2>
          <p className="text-sm text-neutral-1 dark:text-neutral+1">
            Select one of your GitHub repositories to publish posts from. All
            markdown docs in the repository will be converted into blog posts
            and updated on pushes to the primary branch.
          </p>
        </div>

        <div className="">
          <div className="flex flex-col gap-3">
            <fieldset className="flex flex-col gap-2">
              <legend>Repositories</legend>

              {repos?.slice(0, limit).map((repo: GitHubRepo) => (
                <Repo
                  key={repo.id}
                  repo={repo}
                  selected={selectedRepo}
                  setSelected={setSelectedRepo}
                />
              ))}
            </fieldset>
            <div className="flex justify-center">
              {limit < 0 ? (
                <Button
                  onClick={() => {
                    setLimit(9)
                  }}
                  variant={{ intent: "tertiary" }}
                  type="button"
                >
                  Show fewer
                </Button>
              ) : (
                <Button
                  onClick={() => {
                    setLimit(-1)
                  }}
                  variant={{ intent: "tertiary" }}
                  type="button"
                >
                  Show all
                </Button>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

const Repo = ({
  repo,
  selected,
  setSelected,
}: {
  repo: GitHubRepo
  selected: GitHubRepo | null
  setSelected: (repo: GitHubRepo) => void
}) => {
  return (
    <div>
      <input type="radio" name="repo" value={repo.name} />
      <label>{repo.name}</label>
    </div>
  )
}
