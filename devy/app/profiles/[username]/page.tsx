import JSON from "@/components/elements/JSON"

export default async function Profile({
  params,
}: {
  params: { username: string }
}) {
  const profile = null

  if (!profile) return <div>Profile not found</div>

  return (
    <div>
      <JSON raw={profile} />
    </div>
  )
}
