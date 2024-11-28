import type { PageServerLoad } from "./$types"
import { error, type NumericRange } from "@sveltejs/kit"
import db from "$lib/server/db"
import { blog } from "$lib/server/db/schema"


export const load: PageServerLoad = async ({ params, fetch, locals }) => {
  const result = db.select().from(blog).limit(10)

  return {
    hello: "world",
    result
  }
}
