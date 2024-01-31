"use client"

import { useState } from "react"
import * as RadioGroup from "@radix-ui/react-radio-group"
import { useRouter } from "next/navigation"

import Input from "@/components/input"
import Button from "@/components/button"

import useFormState from "./useFormState"
import GitHubRepo from "./GitHubRepo"

interface Props {
  username: string
  repos: GitHubRepo[]
}

export default function Form({ username, repos }: Props) {
  const formState = useFormState(username, repos)
  const router = useRouter()
  const [showAll, setShowAll] = useState(false)
  const limit = 9

  return (
    <>
      <form className="mb-12 flex flex-col gap-4" action={formState.submit}>
        {/* Step 1 */}
        <div>
          <span className="text-xs font-medium text-neutral">Step 1</span>
          <div className="flex flex-col sm:flex-row gap-4 sm:gap-8">
            <div className="w-96 sm:w-60">
              <h2 className="font-medium text-neutral-3 dark:text-neutral+3">
                Name your blog
              </h2>
              <p className="text-sm text-neutral-1 dark:text-neutral+1">
                Pick a display name for your blog. This can be anything you want
                and you can change it later.
              </p>
            </div>

            <div className="flex-1">
              <Input
                className="w-full sm:w-60"
                label="Blog name"
                placeholder="Your blog's name"
                value={formState.fields.blogName}
                setValue={formState.fields.setBlogName}
              />
            </div>
          </div>
        </div>

        {/* Step 2 */}
        <div>
          <span className="text-xs font-medium text-neutral">Step 2</span>

          <div className="flex flex-col sm:flex-row gap-4 sm:gap-8">
            <div className="w-96 sm:w-60">
              <h2 className="font-medium text-neutral-3 dark:text-neutral+3">
                Pick a repository
              </h2>
              <p className="text-sm text-neutral-1 dark:text-neutral+1">
                Select one of your GitHub repositories to publish posts from.
                All markdown docs in the repository will be converted into blog
                posts and updated on pushes to the primary branch.
              </p>
            </div>

            <div className="flex-1">
              <div className="flex flex-col">
                <span className="mb-1 text-sm font-medium">
                  Your public repositories
                </span>

                <RadioGroup.Root className="flex flex-col md:grid md:grid-cols-3 md:gap-1 shadow-sm md:shadow-none">
                  {repos
                    ?.slice(0, showAll ? -1 : limit)
                    .map((repo: GitHubRepo) => (
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
                        onClick={() => formState.fields.setRepoId(repo.id)}
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
              </div>
            </div>
          </div>
        </div>

        {/* Submit */}
        <div className="flex flex-row gap-2 items-start md:ml-[17rem]">
          <Button
            variant={{ action: "create" }}
            disabled={!formState.isComplete}
          >
            Create your blog
          </Button>
          <Button
            variant={{ intent: "tertiary" }}
            type="button"
            onClick={() => router.push("/")}
          >
            Cancel
          </Button>
        </div>
      </form>
    </>
  )
}
