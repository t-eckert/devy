import type { PageServerLoad } from "./$types";
import { clerkClient } from "svelte-clerk/server"

export const load: PageServerLoad = async (event) => {
  const locals = event.locals

  if (locals.auth.userId === null) {
    return { locals }
  }

  const user = await clerkClient.users.getUser(locals.auth.userId)
  console.log(user)
  return {
    locals,

  }
}
