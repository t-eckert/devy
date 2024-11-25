import { DATABASE_URL } from "$env/static/private"
import { drizzle } from "drizzle-orm/node-postgres"

export default drizzle({ connection: DATABASE_URL, casing: "snake_case" })
