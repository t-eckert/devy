import { pgTable, uuid, text, timestamp, integer } from 'drizzle-orm/pg-core';
import { sql } from "drizzle-orm"

const timestamps = {
  updatedAt: timestamp("updated_at"),
  createdAt: timestamp("created_at").defaultNow().notNull(),
}

export const profile = pgTable('profile', {
  id: uuid("id").default(sql`uuid_generate_v4()`).primaryKey().notNull(),
  name: text("name").notNull(),
  username: text("username").notNull(),
  ...timestamps
});

export const blog = pgTable('blog', {
  id: uuid("id").default(sql`uuid_generate_v4()`).primaryKey().notNull(),
  profileId: uuid("profileId").notNull(),
  name: text("name").notNull(),
  ...timestamps
});

export const post = pgTable('post', {
  id: uuid("id").default(sql`uuid_generate_v4()`).primaryKey().notNull(),
  blogId: uuid("blogId").notNull(),
  name: text("name").notNull(),
  likeCount: integer("like_count").default(0),
  ...timestamps
});
