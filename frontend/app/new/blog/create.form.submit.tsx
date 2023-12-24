"use client"

import Button from "@/components/button"

export default function CreateFormSubmit() {
  return (
    <div className="flex flex-row gap-2 items-start md:ml-[17rem]">
      <Button variant={{ action: "create" }}>Create your blog</Button>
      <Button variant={{ intent: "tertiary" }} type="button" onClick={() => {}}>
        Cancel
      </Button>
    </div>
  )
}
