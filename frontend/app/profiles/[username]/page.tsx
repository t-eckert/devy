import api from "@/lib/api"
import Profile from "@/models/Profile"
import User from "@/models/User"
import Member from "@/components/segments/Member"
import Blog from "@/models/Blog"
import Post from "@/models/Post"
import Posts from "@/components/Posts"

interface Props {
  params: {
    username: string
  }
}

export default async function ProfilePage({ params }: Props) {
  const profile = await api.get<Profile>(`/v1/profiles/${params.username}`, 60)
  const user = await api.get<User>(`/v1/users/${params.username}`, 60)
  const blogs = await api.get<Blog[]>(
    `/v1/profiles/${params.username}/blogs`,
    60
  )
  const posts = await api.get<Post[]>(
    `/v1/profiles/${params.username}/posts`,
    60
  )
  const likes = await api.get<Post[]>(
    `/v1/profiles/${params.username}/likes`,
    60
  )

  return (
    <main className="mx-auto my-8 flex flex-row items-start px-2 w-full max-w-6xl gap-4 sm:gap-2">
      <Member profile={profile} user={user} blogs={blogs} />
      <div>
        <div>
          <span>Posts</span>
          <Posts posts={posts} />
        </div>
        <div>
          <span>Likes</span>
          <Posts posts={likes} />
        </div>
      </div>
    </main>
  )
}
