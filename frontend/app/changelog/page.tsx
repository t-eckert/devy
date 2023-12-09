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
    <main className="mx-auto flex flex-col px-2 py-6 sm:py-12 w-full max-w-2xl gap-4 sm:gap-2">
      <h1 className="text-4xl sm:text-6xl sm:mb-4 font-semibold text-neutral-1 dark:text-neutral+1">
        Changelog
      </h1>
      <Markdown content={changelog} />
    </main>
  )
}
