import api from "@/lib/api"
import Profile from "@/models/Profile"
import User from "@/models/User"
import Blog from "@/models/Blog"
import Post from "@/models/Post"
import PostCollection from "@/components/post-collection"
import MemberCard from "@/components/member-card"
import Markdown from "@/components/markdown"

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
    <main className="mx-auto my-8 flex flex-col items-center sm:flex-row sm:items-start px-2 w-full max-w-6xl gap-4 sm:gap-16">
      <MemberCard profile={profile} user={user} blogs={blogs} />

      <div>
        <div className="flex flex-col gap-2 w-full max-w-3xl">
          <Markdown content={profile.bio || ""} />
        </div>

        <div>
          <div className="py-4">
            <h1 className="font-semibold">Posts by {profile.displayName}</h1>
          </div>
          <PostCollection posts={posts} />
        </div>
        <div>
          <div className="py-4">
            <h1 className="font-semibold">Liked by {profile.displayName}</h1>
          </div>
          <PostCollection posts={likes} />
        </div>
      </div>
    </main>
  )
}
