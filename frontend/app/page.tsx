import Json from "@/components/debugging/Json"

import feed from "@/controllers/feed"

export default async function Home() {
  const data = await feed.get.new()

  return (
    <main>
      <Json data={data} />
    </main>
  )
}
