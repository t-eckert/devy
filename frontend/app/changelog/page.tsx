import Markdown from "@/components/markdown"
import changelogController from "@/controllers/changelog"

export default async function Changelog() {
  const changelog = await changelogController.get.fromGitHub()

  if (!changelog) return <div>Changelog not found</div>

  return (
    <main className="mx-auto my-4 flex flex-col sm:flex-row px-2 w-full max-w-6xl gap-4 sm:gap-2">
      <article>
        <Markdown content={changelog} />
      </article>
    </main>
  )
}
