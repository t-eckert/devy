"use client"

import CreateFormStep1 from "./create.form.step-1"
import CreateFormStep2 from "./create.form.step-2"
import CreateFormSubmit from "./create.form.submit"
import useFormState from "./useFormState"
import DataWindow from "@/components/data-window"

export default function CreateForm() {
  const formState = useFormState()

  if (formState.status === "submitting") {
    return <p>Submitting...</p>
  }

  return (
    <>
      <form className="flex flex-col gap-8 lg:gap-10">
        <CreateFormStep1 />
        <CreateFormStep2 />
        <CreateFormSubmit />
      </form>

      <DataWindow data={formState} />
    </>
  )
}
