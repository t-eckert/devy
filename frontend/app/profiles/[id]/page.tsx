import Json from "@/components/debugging/Json"
import Avatar from "@/components/elements/Avatar"

import api from "@/api"
import Profile from "@/models/Profile"
import { Link } from "@/components/elements"

interface Props {
  params: {
    id: string
  }
}

export default async function ProfilePage({ params }: Props) {
  const profile = await api.get<Profile>(`/profiles/${params.id}`, 60)

  if (!profile) return <NotFound />

  return (
    <main className="mx-auto my-8 flex flex-col items-center px-2 w-full max-w-6xl gap-4 sm:gap-2">
      <Avatar {...profile} />
      <Json data={profile} />
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
