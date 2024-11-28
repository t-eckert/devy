import type { Actions } from "./$types"
import { redirect } from "@sveltejs/kit"

export const actions = {
  default: async (event) => {
    event.cookies.delete("token", {
      path: "/"
    })
    redirect(303, "/")
  }
} satisfies Actions
