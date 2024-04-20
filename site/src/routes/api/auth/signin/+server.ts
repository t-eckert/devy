import type { RequestHandler } from "./$types"
import { redirect } from "@sveltejs/kit"
import config from "$lib/config"

export const GET: RequestHandler = () => {
  throw redirect(307, `${config.api}/auth/login`)
}
