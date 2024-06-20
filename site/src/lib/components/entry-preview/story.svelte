<script lang="ts">
	import type { Entry } from "$lib/types"
	import Preview from "./preview.svelte"
	import Author from "./segments/author.svelte"
	import Title from "./segments/title.svelte"
	import Likes from "./segments/likes.svelte"
	import Bookmark from "./segments/bookmark.svelte"
	import Blog from "./segments/blog.svelte"
	import Date from "./segments/date.svelte"

	export let entry: Entry
	const { postSlug, title, blogSlug, blogName, authorName, authorSlug, likes, createdAt } = entry
</script>

<div class="flex flex-col gap-8">
	<Preview>
		<Title slot="title" {title} {blogSlug} {postSlug} />
	</Preview>

	<Preview>
		<Blog slot="blog" {blogName} {blogSlug} />
		<Title slot="title" {title} {blogSlug} {postSlug} />
	</Preview>

	<Preview>
		<Author slot="author" {authorName} {authorSlug} />
		<Title slot="title" {title} {blogSlug} {postSlug} />
	</Preview>

	<Preview>
		<Author slot="author" {authorName} {authorSlug} />
		<Blog slot="blog" {blogName} {blogSlug} />
		<Title slot="title" {title} {blogSlug} {postSlug} />
	</Preview>

	<Preview>
		<Author slot="author" {authorName} {authorSlug} />
		<Date slot="date" date={createdAt} />
		<Title slot="title" {title} {blogSlug} {postSlug} />
	</Preview>

	<Preview>
		<Author slot="author" {authorName} {authorSlug} />
		<Date slot="date" date={createdAt} />
		<Blog slot="blog" {blogName} {blogSlug} />
		<Title slot="title" {title} {blogSlug} {postSlug} />
	</Preview>

	<Preview>
		<Author slot="author" {authorName} {authorSlug} />
		<Blog slot="blog" {blogName} {blogSlug} />
		<Date slot="date" date={createdAt} />
		<Title slot="title" {title} {blogSlug} {postSlug} />
	</Preview>

	<Preview>
		<Author slot="author" {authorName} {authorSlug} />
		<Date slot="date" date={createdAt} />
		<Bookmark slot="bookmark" />
		<Blog slot="blog" {blogName} {blogSlug} />
		<Title slot="title" {title} {blogSlug} {postSlug} />
	</Preview>

	<Preview>
		<Author slot="author" {authorName} {authorSlug} />
		<Bookmark slot="bookmark" />
		<Blog slot="blog" {blogName} {blogSlug} />
		<Date slot="date" date={createdAt} />
		<Title slot="title" {title} {blogSlug} {postSlug} />
	</Preview>

	<Preview>
		<Title slot="title" {title} {blogSlug} {postSlug} />
		<Likes slot="likes" liked={true} {likes} />
	</Preview>

	<Preview>
		<Author slot="author" {authorName} {authorSlug} />
		<Likes slot="likes" liked={true} {likes} />
		<Bookmark slot="bookmark" />
		<Blog slot="blog" {blogName} {blogSlug} />
		<Date slot="date" date={createdAt} />
		<Title slot="title" {title} {blogSlug} {postSlug} />
	</Preview>
</div>
