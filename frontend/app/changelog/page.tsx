import Markdown from "@/components/markdown"

export default async function Changelog() {
  const res = await fetch(
    "https://raw.githubusercontent.com/t-eckert/devy/main/CHANGELOG.md"
  )
  const text = await res.text()

  return (
    <main className="mx-auto my-4 flex flex-col sm:flex-row px-2 w-full max-w-6xl gap-4 sm:gap-2">
      <article>
        <Markdown content={text} />
      </article>
    </main>
  )
}
