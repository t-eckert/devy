import { pgTable, unique, uuid, timestamp, text, jsonb, foreignKey, boolean, bigint, integer, index, primaryKey } from "drizzle-orm/pg-core"
import { sql } from "drizzle-orm"

export const user = pgTable("user", {
  id: uuid().default(sql`uuid_generate_v4()`).primaryKey().notNull(),
  createdAt: timestamp("created_at", { withTimezone: true, mode: 'string' }).defaultNow().notNull(),
  updatedAt: timestamp("updated_at", { withTimezone: true, mode: 'string' }).defaultNow().notNull(),
  username: text().notNull(),
  email: text(),
  githubUsername: text("github_username"),
  role: text().default('user').notNull(),
  status: text().default('active').notNull(),
  metadata: jsonb().default({}),
}, (table) => {
  return {
    userUsernameKey: unique("user_username_key").on(table.username),
    userEmailKey: unique("user_email_key").on(table.email),
    userGithubUsernameKey: unique("user_github_username_key").on(table.githubUsername),
  }
});

export const profile = pgTable("profile", {
  id: uuid().default(sql`uuid_generate_v4()`).primaryKey().notNull(),
  userId: uuid("user_id").notNull(),
  createdAt: timestamp("created_at", { withTimezone: true, mode: 'string' }).defaultNow().notNull(),
  updatedAt: timestamp("updated_at", { withTimezone: true, mode: 'string' }).defaultNow().notNull(),
  displayName: text("display_name"),
  bio: text(),
  avatarUrl: text("avatar_url"),
  websiteUrl: text("website_url"),
  twitterUsername: text("twitter_username"),
  githubUsername: text("github_username"),
  status: text().default('active').notNull(),
  visibility: text().default('public').notNull(),
  isDeleted: boolean("is_deleted").default(false).notNull(),
  isLocked: boolean("is_locked").default(false).notNull(),
  isFeatured: boolean("is_featured").default(false).notNull(),
  isBot: boolean("is_bot").default(false).notNull(),
  isSponsor: boolean("is_sponsor").default(false).notNull(),
  blueskyUrl: text("bluesky_url"),
  linkedinUrl: text("linkedin_url"),
}, (table) => {
  return {
    profileUserIdFkey: foreignKey({
      columns: [table.userId],
      foreignColumns: [user.id],
      name: "profile_user_id_fkey"
    }).onDelete("cascade"),
    uniqueUserId: unique("unique_user_id").on(table.userId),
  }
});

export const repo = pgTable("repo", {
  id: uuid().default(sql`uuid_generate_v4()`).primaryKey().notNull(),
  url: text().notNull(),
  blogId: uuid("blog_id").notNull(),
  // You can use { mode: "bigint" } if numbers are exceeding js number limitations
  githubId: bigint("github_id", { mode: "number" }),
  githubName: text("github_name"),
  metadata: jsonb().default({}).notNull(),
  createdAt: timestamp("created_at", { withTimezone: true, mode: 'string' }).defaultNow().notNull(),
  updatedAt: timestamp("updated_at", { withTimezone: true, mode: 'string' }).defaultNow().notNull(),
}, (table) => {
  return {
    repoUrlKey: unique("repo_url_key").on(table.url),
    repoBlogIdKey: unique("repo_blog_id_key").on(table.blogId),
    repoGithubIdKey: unique("repo_github_id_key").on(table.githubId),
  }
});

export const feed = pgTable("feed", {
  id: uuid().default(sql`uuid_generate_v4()`).primaryKey().notNull(),
  name: text().notNull(),
  createdAt: timestamp("created_at", { withTimezone: true, mode: 'string' }).defaultNow().notNull(),
  updatedAt: timestamp("updated_at", { withTimezone: true, mode: 'string' }).defaultNow().notNull(),
  filterTags: text("filter_tags").array().default([""]),
  filterProfiles: integer("filter_profiles").array().default([]),
  filterBlogs: integer("filter_blogs").array().default([]),
  orderBy: text("order_by").default('created_at'),
});

export const session = pgTable("session", {
  id: uuid().default(sql`uuid_generate_v4()`).primaryKey().notNull(),
  userId: uuid("user_id").notNull(),
  metadata: jsonb().default({}).notNull(),
  createdAt: timestamp("created_at", { withTimezone: true, mode: 'string' }).defaultNow().notNull(),
  lastUsedAt: timestamp("last_used_at", { withTimezone: true, mode: 'string' }).defaultNow().notNull(),
  exp: integer().default(3600).notNull(),
  accessToken: text("access_token").notNull(),
  encodingKey: text("encoding_key").notNull(),
}, (table) => {
  return {
    sessionUserIdFkey: foreignKey({
      columns: [table.userId],
      foreignColumns: [user.id],
      name: "session_user_id_fkey"
    }).onDelete("cascade"),
  }
});

export const blog = pgTable("blog", {
  id: uuid().default(sql`uuid_generate_v4()`).primaryKey().notNull(),
  profileId: uuid("profile_id").notNull(),
  createdAt: timestamp("created_at", { withTimezone: true, mode: 'string' }).defaultNow().notNull(),
  updatedAt: timestamp("updated_at", { withTimezone: true, mode: 'string' }).defaultNow().notNull(),
  name: text().notNull(),
  slug: text().notNull(),
  description: text(),
  repoUrl: text("repo_url").notNull(),
}, (table) => {
  return {
    idxBlogSlug: index("idx_blog_slug").using("btree", table.slug.asc().nullsLast().op("text_ops")),
    blogProfileIdFkey: foreignKey({
      columns: [table.profileId],
      foreignColumns: [profile.id],
      name: "blog_profile_id_fkey"
    }).onDelete("cascade"),
    blogSlugKey: unique("blog_slug_key").on(table.slug),
  }
});

export const post = pgTable("post", {
  id: uuid().default(sql`uuid_generate_v4()`).primaryKey().notNull(),
  blogId: uuid("blog_id").notNull(),
  createdAt: timestamp("created_at", { withTimezone: true, mode: 'string' }).defaultNow().notNull(),
  updatedAt: timestamp("updated_at", { withTimezone: true, mode: 'string' }).defaultNow().notNull(),
  title: text().notNull(),
  slug: text().notNull(),
  body: text().notNull(),
  likeCount: integer("like_count").default(0),
  isDraft: boolean("is_draft").default(false).notNull(),
}, (table) => {
  return {
    postBlogIdFkey: foreignKey({
      columns: [table.blogId],
      foreignColumns: [blog.id],
      name: "post_blog_id_fkey"
    }).onDelete("cascade"),
    postSlugKey: unique("post_slug_key").on(table.slug),
  }
});

export const upload = pgTable("upload", {
  id: uuid().default(sql`uuid_generate_v4()`).primaryKey().notNull(),
  createdAt: timestamp("created_at", { withTimezone: true, mode: 'string' }).defaultNow().notNull(),
  updatedAt: timestamp("updated_at", { withTimezone: true, mode: 'string' }).defaultNow().notNull(),
  previousUploadId: uuid("previous_upload_id"),
  status: text().default('pending').notNull(),
  repo: text().notNull(),
  logs: text().array().default([""]),
  sha: text().default('HEAD').notNull(),
  diff: text(),
  changeset: jsonb(),
  blogId: uuid("blog_id").notNull(),
}, (table) => {
  return {
    uploadPreviousUploadIdFkey: foreignKey({
      columns: [table.previousUploadId],
      foreignColumns: [table.id],
      name: "upload_previous_upload_id_fkey"
    }),
    uploadBlogIdFkey: foreignKey({
      columns: [table.blogId],
      foreignColumns: [blog.id],
      name: "upload_blog_id_fkey"
    }).onDelete("cascade"),
  }
});

export const tag = pgTable("tag", {
  value: text().primaryKey().notNull(),
});

export const webhook = pgTable("webhook", {
  id: uuid().default(sql`uuid_generate_v4()`).primaryKey().notNull(),
  receivedAt: timestamp("received_at", { withTimezone: true, mode: 'string' }).defaultNow().notNull(),
  type: text().notNull(),
  payload: jsonb().notNull(),
});

export const postTag = pgTable("post_tag", {
  postId: uuid("post_id").notNull(),
  tag: text().notNull(),
}, (table) => {
  return {
    postTagPostIdFkey: foreignKey({
      columns: [table.postId],
      foreignColumns: [post.id],
      name: "post_tag_post_id_fkey"
    }).onDelete("cascade"),
    postTagTagFkey: foreignKey({
      columns: [table.tag],
      foreignColumns: [tag.value],
      name: "post_tag_tag_fkey"
    }).onDelete("cascade"),
    postTagPkey: primaryKey({ columns: [table.postId, table.tag], name: "post_tag_pkey" }),
  }
});

export const like = pgTable("like", {
  profileId: uuid("profile_id").notNull(),
  postId: uuid("post_id").notNull(),
  createdAt: timestamp("created_at", { withTimezone: true, mode: 'string' }).defaultNow().notNull(),
}, (table) => {
  return {
    likeProfileIdFkey: foreignKey({
      columns: [table.profileId],
      foreignColumns: [profile.id],
      name: "like_profile_id_fkey"
    }).onDelete("cascade"),
    likePostIdFkey: foreignKey({
      columns: [table.postId],
      foreignColumns: [post.id],
      name: "like_post_id_fkey"
    }).onDelete("cascade"),
    likePkey: primaryKey({ columns: [table.profileId, table.postId], name: "like_pkey" }),
  }
});

export const bookmark = pgTable("bookmark", {
  profileId: uuid("profile_id").notNull(),
  postId: uuid("post_id").notNull(),
  createdAt: timestamp("created_at", { withTimezone: true, mode: 'string' }).defaultNow().notNull(),
}, (table) => {
  return {
    bookmarkProfileIdFkey: foreignKey({
      columns: [table.profileId],
      foreignColumns: [profile.id],
      name: "bookmark_profile_id_fkey"
    }).onDelete("cascade"),
    bookmarkPostIdFkey: foreignKey({
      columns: [table.postId],
      foreignColumns: [post.id],
      name: "bookmark_post_id_fkey"
    }).onDelete("cascade"),
    bookmarkPkey: primaryKey({ columns: [table.profileId, table.postId], name: "bookmark_pkey" }),
  }
});

export const follow = pgTable("follow", {
  profileId: uuid("profile_id").notNull(),
  blogId: uuid("blog_id").notNull(),
  createdAt: timestamp("created_at", { withTimezone: true, mode: 'string' }).defaultNow().notNull(),
}, (table) => {
  return {
    followProfileIdFkey: foreignKey({
      columns: [table.profileId],
      foreignColumns: [profile.id],
      name: "follow_profile_id_fkey"
    }).onDelete("cascade"),
    followBlogIdFkey: foreignKey({
      columns: [table.blogId],
      foreignColumns: [blog.id],
      name: "follow_blog_id_fkey"
    }).onDelete("cascade"),
    followPkey: primaryKey({ columns: [table.profileId, table.blogId], name: "follow_pkey" }),
  }
});
