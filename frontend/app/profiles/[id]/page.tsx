import Json from "@/components/debugging/Json"

interface Props {
  params: {
    id: string
  }
}

export default async function Profile({ params }: Props) {
  return (
    <main className="mx-auto my-8 flex flex-col sm:flex-row px-2 w-full max-w-6xl gap-4 sm:gap-2">
      <Json data={params} />
    </main>
  )
}
