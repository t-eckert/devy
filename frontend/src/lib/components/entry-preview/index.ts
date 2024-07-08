import EntryPreview from "./entry-preview.svelte"
import EntryPreviewAuthor from "./entry-preview.author.svelte"
import EntryPreviewBlog from "./entry-preview.blog.svelte"
import EntryPreviewBookmark from "./entry-preview.bookmark.svelte"
import EntryPreviewDate from "./entry-preview.date.svelte"
import EntryPreviewLikes from "./entry-preview.likes.svelte"
import EntryPreviewTitle from "./entry-preview.title.svelte"

EntryPreview.Author = EntryPreviewAuthor
EntryPreview.Blog = EntryPreviewBlog
EntryPreview.Bookmark = EntryPreviewBookmark
EntryPreview.Date = EntryPreviewDate
EntryPreview.Likes = EntryPreviewLikes
EntryPreview.Title = EntryPreviewTitle

export default EntryPreview
