import Json from "@/components/debugging/Json"
import Avatar from "@/components/elements/Avatar"

import profileController from "@/controllers/profile"

interface Props {
  params: {
    id: string
  }
}

export default async function Profile({ params }: Props) {
  const profile = await profileController.get.byId(params.id)

  if (!profile) return <div>Profile not found</div>

  return (
    <main className="mx-auto my-8 flex flex-col items-center px-2 w-full max-w-6xl gap-4 sm:gap-2">
      <Avatar {...profile} />
      <Json data={profile} />
    </main>
  )
}
