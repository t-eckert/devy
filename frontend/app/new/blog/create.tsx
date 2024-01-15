"use client"

import useCreateState from "./useCreateState"
import CreateLoggedOut from "./create.logged-out"
import CreateForm from "./create.form"

export default function Create() {
  const state = useCreateState()

  state.userBlogs = []

  if (state.session === null || state.session === undefined) {
    return <CreateLoggedOut />
  }

  if (state.userBlogs && state.userBlogs.length > 0) {
    return (
      <p>
        You already have a blog. While this project is in preview, every user
        can only have one blog.
      </p>
    )
  }

  if (state.userBlogs && state.userBlogs.length === 0) {
    return <CreateForm />
  }

  return <></>
}
