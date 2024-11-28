import 'dotenv/config';
import { defineConfig } from "drizzle-kit"

export default defineConfig({
  dialect: "postgresql",
  schema: "./src/lib/server/db/schema.ts",
  casing: "snake_case",
  out: "migrations",
  dbCredentials: {
    //@ts-expect-error unknown process type
    url: process.env.DATABASE_URL,
  },
})
