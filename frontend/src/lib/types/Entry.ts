export default interface Entry {
  id: string
  postSlug: string
  title: string
  body: string
  likeCount: number

  blogSlug: string
  blogName: string

  authorSlug: string
  authorName: string

  createdAt: string
  updatedAt: string

  liked: boolean
  bookmarked: boolean
}
