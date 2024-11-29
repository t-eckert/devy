import type { PageServerLoad } from "./$types"
import db from "$lib/server/db"
import { profile } from "$lib/server/db/schema"
import { eq } from "drizzle-orm"

export const load: PageServerLoad = async ({ params }) => {

  return {
    profile: await db.select().from(profile).where(eq(profile.username, params.username))
  }
}
