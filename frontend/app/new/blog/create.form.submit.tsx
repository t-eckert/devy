"use client"

import { useRouter } from "next/navigation"

import Button from "@/components/button"

import useFormState from "./useFormState"

export default function CreateFormSubmit() {
  const formState = useFormState()
  const router = useRouter()

  return (
    <div className="flex flex-row gap-2 items-start md:ml-[17rem]">
      <Button
        variant={{ action: "create" }}
        disabled={!formState.isComplete}
        onClick={formState.submit}
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
  )
}
