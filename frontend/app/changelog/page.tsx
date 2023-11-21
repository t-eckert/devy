import Markdown from "@/components/markdown"

export default async function ChangelogPage() {
  const res = await fetch(
    "https://raw.githubusercontent.com/t-eckert/devy/main/CHANGELOG.md",
    {
      next: {
        revalidate: 3600,
      },
    }
  )
  const changelog = await res.text()

  return (
    <main className="mx-auto my-4 flex flex-col sm:flex-row px-2 w-full max-w-6xl gap-4 sm:gap-2">
      <article className="mx-auto w-full max-w-xl">
        <Markdown content={changelog} />
      </article>
    </main>
  )
}
