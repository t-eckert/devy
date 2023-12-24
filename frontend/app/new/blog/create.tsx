"use client"

import useCreateState from "./useCreateState"
import CreateLoggedOut from "./create.logged-out"
import CreateForm from "./create.form"
import Json from "@/components/json"

export default function Create() {
  const state = useCreateState()

  if (state.session === null) {
    return (
      <>
        <Json data={state} />
        <CreateLoggedOut />
      </>
    )
  }

  return (
    <>
      {/* <Json data={state} /> */}
      <CreateForm />
    </>
  )
}
