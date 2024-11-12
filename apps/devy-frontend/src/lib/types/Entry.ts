export default interface Entry {
  id: string
  slug: string

  title: string
  body: string
  likeCount: number
  coverImage?: string

  blogSlug: string
  blogName: string

  authorSlug: string
  authorName: string

  createdAt: string
  updatedAt: string

  isDraft: boolean
}
