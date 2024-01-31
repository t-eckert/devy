import Markdown from "@/components/markdown"

import data from "./data"

export default async function ChangelogPage() {
  const { changelog } = await data()

  return (
    <main className="mx-auto flex flex-col px-3 py-6 sm:py-12 w-full max-w-2xl gap-4 sm:gap-2">
      <h1 className="text-4xl mb-2 sm:text-6xl sm:mb-4 font-semibold text-neutral-1 dark:text-neutral+1">
        Changelog
      </h1>
      <Markdown content={changelog} />
    </main>
  )
}
