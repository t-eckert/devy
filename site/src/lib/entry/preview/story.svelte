<script lang="ts">
	import type { Entry } from "$lib/types"
	import Preview from "./preview.svelte"
	import Author from "./segments/author.svelte"
	import Title from "./segments/title.svelte"
	import Likes from "./segments/likes.svelte"
	import Bookmark from "./segments/bookmark.svelte"
	import Blog from "./segments/blog.svelte"
	import Date from "./segments/date.svelte"

	import RelativeDate from "$lib/utils/relative-date.svelte"
	import { Heart, Bookmark as BookmarkIcon } from "radix-icons-svelte"

	export let entry: Entry
	const { postSlug, title, blogSlug, blogName, authorName, authorSlug, likes, createdAt } = entry
</script>

<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-8 items-start justify-start">
	<Preview {entry}>
		<Title slot="title" />
	</Preview>

	<Preview {entry}>
		<Blog slot="blog" />
		<Title slot="title" />
	</Preview>

	<Preview {entry}>
		<Author slot="author" />
		<Title slot="title" />
	</Preview>

	<Preview {entry}>
		<Author slot="author" />
		<Blog slot="blog" />
		<Title slot="title" />
	</Preview>

	<Preview {entry}>
		<Author slot="author" />
		<Date slot="date" />
		<Title slot="title" />
	</Preview>

	<Preview {entry}>
		<Author slot="author" />
		<Date slot="date" />
		<Blog slot="blog" />
		<Title slot="title" />
	</Preview>

	<Preview {entry}>
		<Author slot="author" />
		<Blog slot="blog" />
		<Date slot="date" />
		<Title slot="title" />
	</Preview>

	<Preview {entry}>
		<Author slot="author" />
		<Date slot="date" />
		<Bookmark slot="bookmark" />
		<Blog slot="blog" />
		<Title slot="title" />
	</Preview>

	<Preview {entry}>
		<Author slot="author" />
		<Bookmark slot="bookmark" />
		<Blog slot="blog" />
		<Date slot="date" />
		<Title slot="title" />
	</Preview>

	<Preview {entry}>
		<Title slot="title" />
		<Likes slot="likes" />
	</Preview>

	<Preview {entry}>
		<Author slot="author" />
		<Likes slot="likes" />
		<Bookmark slot="bookmark" />
		<Blog slot="blog" />
		<Date slot="date" />
		<Title slot="title" />
	</Preview>
</div>
