import { pgTable, uuid, text } from 'drizzle-orm/pg-core';
import { sql } from "drizzle-orm"

export const profile = pgTable('profile', {
  id: uuid("id").default(sql`uuid_generate_v4()`).primaryKey().notNull(),
  name: text("name").notNull(),
});

export const blog = pgTable('blog', {
  id: uuid("id").default(sql`uuid_generate_v4()`).primaryKey().notNull(),
  name: text("name").notNull(),
});

export const post = pgTable('post', {
  id: uuid("id").default(sql`uuid_generate_v4()`).primaryKey().notNull(),
  blogId: uuid("blogId").notNull(),
  name: text("name").notNull(),
});
