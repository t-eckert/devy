import api from "@/lib/api"
import { Blog, Post } from "@/models"

interface Props {
  blog: Blog
  posts: Post[]
}

export default async function data(blogSlug: string): Promise<Props> {
  const blog = await api.get<Blog>(`/v1/blogs/${blogSlug}`, 600)
  const posts = await api.get<Post[]>(`/v1/blogs/${blogSlug}/posts`, 600)

  return {
    blog,
    posts,
  }
}
