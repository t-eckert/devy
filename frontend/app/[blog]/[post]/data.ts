import { Post } from "@/models";
import api from "@/lib/api"

interface Data {
  post: Post
}

export default async function data(blogSlug: string, postSlug: string): Promise<Data> {
  const post = await api.get<Post>(
    `/v1/blogs/${blogSlug}/posts/${postSlug}`,
    600
  )

  return {
    post,
  }
}

