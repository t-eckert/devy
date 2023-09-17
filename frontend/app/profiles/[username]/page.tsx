import Json from "@/components/debugging/Json"

import api from "@/api"
import Profile from "@/models/Profile"
import { Link } from "@/components/elements"
import User from "@/models/User"
import Member from "@/components/segments/Member"
import Blog from "@/models/Blog"

interface Props {
  params: {
    username: string
  }
}

export default async function ProfilePage({ params }: Props) {
  const profile = await api.get<Profile>(`/profiles/${params.username}`, 60)
  const user = await api.get<User>(`/users/${params.username}`, 60)
  const blogs = await api.get<Blog[]>(`/profiles/${params.username}/blogs`, 60)

  if (!profile || !user || !blogs) return <NotFound />

  return (
    <main className="mx-auto my-8 flex flex-row items-start px-2 w-full max-w-6xl gap-4 sm:gap-2">
      <Member profile={profile} user={user} blogs={blogs} />
    </main>
  )
}

const NotFound = () => (
  <main className="mx-auto my-8 flex flex-col items-center px-2 py-24 w-full max-w-6xl gap-4 sm:gap-2">
    <div className="flex flex-col items-center">
      <p>Profile not found.</p>
      <Link href="/" variant={{ underline: true }}>
        Return home
      </Link>
    </div>
  </main>
)
