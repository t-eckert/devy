"use client"

import useCreateState from "./useCreateState"
import Input from "@/components/input"

export default function CreateFormStep1() {
  const { blogName, setBlogName, session } = useCreateState()

  return (
    <div>
      <span className="text-xs font-medium text-neutral">Step 1</span>
      <div className="flex flex-col sm:flex-row gap-4 sm:gap-8">
        <div className="w-96 sm:w-60">
          <h2 className="font-medium text-neutral-3 dark:text-neutral+3">
            Name your blog
          </h2>
          <p className="text-sm text-neutral-1 dark:text-neutral+1">
            Pick a display name for your blog. This can be anything you want and
            you can change it later.
          </p>
        </div>

        <div className="flex-1">
          <Input
            className="w-full sm:w-60"
            label="Blog name"
            value={blogName}
            setValue={setBlogName}
            placeholder={`${
              session ? session?.profile.displayName + "'s" : "Your"
            } Blog`}
          />
        </div>
      </div>
    </div>
  )
}
