"use client"

import { useState } from "react"

import * as RadioGroup from "@radix-ui/react-radio-group"

import Button from "@/components/button"

import useCreateState from "./useCreateState"
import GitHubRepo from "./GitHubRepo"
import useFormState from "./useFormState"

export default function CreateFormStep2() {
  const { repos, isLoadingRepos } = useCreateState()

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

        <div className="flex-1">
          <div className="flex flex-col">
            <span className="mb-1 text-sm font-medium">
              Your public repositories
            </span>

            {isLoadingRepos || repos === undefined ? (
              <Loading />
            ) : (
              <Repos repos={repos} />
            )}
          </div>
        </div>
      </div>
    </div>
  )
}

const Loading = () => (
  <div className="flex flex-col md:grid md:grid-cols-3 md:gap-1 shadow-sm md:shadow-none">
    {Array.from({ length: 9 }).map((_, i) => (
      <div
        key={i}
        className={[
          "p-2 md:h-32 first:rounded-t md:rounded border",
          "border-t-neutral+1 border-x-neutral+1 border-b-neutral+3 md:border-neutral+1",
          "dark:border-t-neutral-1 dark:border-x-neutral-1 dark:border-b-neutral-3 dark:md:border-neutral-1 dark:bg-neutral-1",
          "flex flex-col items-start transition-all",
          "animate-pulse",
        ].join(" ")}
      />
    ))}
  </div>
)

const Repos = ({ repos }: { repos: GitHubRepo[] }) => {
  const formState = useFormState()
  const [showAll, setShowAll] = useState(false)
  const limit = 9

  return (
    <RadioGroup.Root className="flex flex-col md:grid md:grid-cols-3 md:gap-1 shadow-sm md:shadow-none">
      {repos?.slice(0, showAll ? -1 : limit).map((repo: GitHubRepo) => (
        <RadioGroup.Item
          className={[
            "p-2 md:h-32 first:rounded-t md:rounded border",
            "border-t-neutral+1 border-x-neutral+1 border-b-neutral+3 md:border-neutral+1",
            "dark:border-t-neutral-1 dark:border-x-neutral-1 dark:border-b-neutral-3 dark:md:border-neutral-1",
            "flex flex-col items-start transition-all",
            "data-[state=checked]:border-blue-primary",
            "dark:data-[state=checked]:border-blue-primary",
          ].join(" ")}
          value={repo.name}
          key={repo.id}
          onClick={() => formState.setRepo(repo)}
        >
          <RadioGroup.Indicator />
          <span className="font-medium text-neutral-3 dark:text-neutral+3">
            {repo.name}
          </span>
          <p className="hidden md:block text-left text-sm text-neutral-1 dark:text-neutral+1">
            {repo.description}
          </p>
        </RadioGroup.Item>
      ))}

      <span
        className={[
          "py-2 col-span-full border border-neutral+1 rounded-b",
          "bg-neutral+1 md:bg-neutral+3 md:border-neutral+3",
          "dark:bg-neutral-1 dark:md:bg-neutral-2 dark:md:border-neutral-2",
          "md:mt-4 w-full flex items-center justify-center",
        ].join(" ")}
      >
        <Button
          onClick={() => setShowAll(!showAll)}
          variant={{ intent: "tertiary" }}
          type="button"
        >
          {showAll ? "Show fewer" : "Show all repos"}
        </Button>
      </span>
    </RadioGroup.Root>
  )
}
