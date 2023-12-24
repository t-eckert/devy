"use client"

import Button from "@/components/button"

import useCreateState from "./useCreateState"

export default function CreateFormSubmit() {
  const { isSubmittable, onSubmit } = useCreateState()

  return (
    <>
      <div className="col-span-4 border-b border-b-neutral+1 dark:border-b-neutral-1" />
      <div className="col-start-2 pl-3">
        <Button
          onSubmit={onSubmit}
          disabled={!isSubmittable}
          variant={{ action: "create" }}
        >
          Create your blog
        </Button>
      </div>
    </>
  )
}
