import Link from "@/components/link"

export default function About() {
  return (
    <section className="mx-auto my-4 px-2 w-full max-w-6xl items-start flex flex-col gap-4">
      <h1 className="text-4xl font-semibold text-neutral-1 dark:text-neutral+1">
        Markdown is the medium
      </h1>

      <p className="mb-2 w-full max-w-md">
        Devy lets you blog from a GitHub repo. Every markdown file is converted
        to a blog post and automatically updated when you push changes.
      </p>

      <Link href="/new/blog" asButton>
        Create a new blog
      </Link>
    </section>
  )
}
