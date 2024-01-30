import api from "@/lib/api"
import { User, Blog, Post, Profile } from "@/models"

interface Data {
  profile: Profile
  user: User
  blogs: Blog[]
  posts: Post[]
  likes: Post[]
}

export default async function data(username: string): Promise<Data> {
  const profile = await api.get<Profile>(`/v1/profiles/${username}`, 60)
  const user = await api.get<User>(`/v1/users/${username}`, 60)
  const blogs = await api.get<Blog[]>(`/v1/profiles/${username}/blogs`, 60)
  const posts = await api.get<Post[]>(`/v1/profiles/${username}/posts`, 60)
  const likes = await api.get<Post[]>(`/v1/profiles/${username}/likes`, 60)

  return {
    profile,
    user,
    blogs,
    posts,
    likes,
  }
}
