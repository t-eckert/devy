"use client"

import CreateFormStep1 from "./create.form.step-1"
import CreateFormStep2 from "./create.form.step-2"
import CreateFormSubmit from "./create.form.submit"

export default function CreateForm() {
  return (
    <form className="flex flex-col gap-8 lg:gap-10">
      <CreateFormStep1 />
      <CreateFormStep2 />
      <CreateFormSubmit />
    </form>
  )
}
