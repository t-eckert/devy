import type { SvelteComponentTyped } from "svelte"
import type EntryPreviewAuthor from "./entry-preview.author.svelte"
import type EntryPreviewBlog from "./entry-preview.blog.svelte"
import type EntryPreviewBookmark from "./entry-preview.bookmark.svelte"
import type EntryPreviewDate from "./entry-preview.date.svelte"
import type EntryPreviewLikes from "./entry-preview.likes.svelte"
import type EntryPreviewTitle from "./entry-preview.title.svelte"

declare module "./entry-preview.svelte" {
	export interface EntryPreviewType extends SvelteComponentTyped {
		Author: typeof EntryPreviewAuthor
		Blog: typeof EntryPreviewBlog
		Bookmark: typeof EntryPreviewBookmark
		Date: typeof EntryPreviewDate
		Likes: typeof EntryPreviewLikes
		Title: typeof EntryPreviewTitle
	}

	const EntryPreview: EntryPreviewType
	export default EntryPreview
}
