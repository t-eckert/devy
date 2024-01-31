import PostCollection from "@/components/post/post-collection"
import MemberCard from "@/components/profile/member-card"

import data from "./data"

interface Props {
  params: {
    username: string
  }
}

export default async function ProfilePage({ params }: Props) {
  const { profile, user, blogs, posts, likes } = await data(params.username)

  return (
    <main className="mx-auto my-8 flex flex-col items-center sm:flex-row sm:items-start px-2 w-full max-w-6xl gap-4 sm:gap-16">
      <MemberCard profile={profile} user={user} blogs={blogs} />

      <div>
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
