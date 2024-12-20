export default interface Post {
  id: string
  postSlug: string
  title: string
  body: string
  likeCount: number
  isDraft: boolean

  blogSlug: string
  blogName: string

  authorSlug: string
  authorName: string

  createdAt: string
  updatedAt: string

  liked: boolean
  bookmarked: boolean
}
