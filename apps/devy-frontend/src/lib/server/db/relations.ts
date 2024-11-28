import { relations } from "drizzle-orm/relations";
import { user, profile, session, blog, post, upload, postTag, tag, like, bookmark, follow } from "./schema";

export const profileRelations = relations(profile, ({one, many}) => ({
	user: one(user, {
		fields: [profile.userId],
		references: [user.id]
	}),
	blogs: many(blog),
	likes: many(like),
	bookmarks: many(bookmark),
	follows: many(follow),
}));

export const userRelations = relations(user, ({many}) => ({
	profiles: many(profile),
	sessions: many(session),
}));

export const sessionRelations = relations(session, ({one}) => ({
	user: one(user, {
		fields: [session.userId],
		references: [user.id]
	}),
}));

export const blogRelations = relations(blog, ({one, many}) => ({
	profile: one(profile, {
		fields: [blog.profileId],
		references: [profile.id]
	}),
	posts: many(post),
	uploads: many(upload),
	follows: many(follow),
}));

export const postRelations = relations(post, ({one, many}) => ({
	blog: one(blog, {
		fields: [post.blogId],
		references: [blog.id]
	}),
	postTags: many(postTag),
	likes: many(like),
	bookmarks: many(bookmark),
}));

export const uploadRelations = relations(upload, ({one, many}) => ({
	upload: one(upload, {
		fields: [upload.previousUploadId],
		references: [upload.id],
		relationName: "upload_previousUploadId_upload_id"
	}),
	uploads: many(upload, {
		relationName: "upload_previousUploadId_upload_id"
	}),
	blog: one(blog, {
		fields: [upload.blogId],
		references: [blog.id]
	}),
}));

export const postTagRelations = relations(postTag, ({one}) => ({
	post: one(post, {
		fields: [postTag.postId],
		references: [post.id]
	}),
	tag: one(tag, {
		fields: [postTag.tag],
		references: [tag.value]
	}),
}));

export const tagRelations = relations(tag, ({many}) => ({
	postTags: many(postTag),
}));

export const likeRelations = relations(like, ({one}) => ({
	profile: one(profile, {
		fields: [like.profileId],
		references: [profile.id]
	}),
	post: one(post, {
		fields: [like.postId],
		references: [post.id]
	}),
}));

export const bookmarkRelations = relations(bookmark, ({one}) => ({
	profile: one(profile, {
		fields: [bookmark.profileId],
		references: [profile.id]
	}),
	post: one(post, {
		fields: [bookmark.postId],
		references: [post.id]
	}),
}));

export const followRelations = relations(follow, ({one}) => ({
	profile: one(profile, {
		fields: [follow.profileId],
		references: [profile.id]
	}),
	blog: one(blog, {
		fields: [follow.blogId],
		references: [blog.id]
	}),
}));