import Link from "@/components/link"

export default function About() {
  return (
    <section className="mx-auto my-4 px-2 w-full max-w-6xl items-start flex flex-col gap-4">
      <h1 className="text-4xl font-semibold text-neutral-1 dark:text-neutral+1">
        Markdown is the medium
      </h1>

      <p className="w-full max-w-md">
        Devy lets you blog from a GitHub repo. Every markdown file is converted
        to a blog post and automatically updated when you push changes.
      </p>

      <Link href="/new/blog" variant={{ underline: false }} className={cta}>
        Create a new blog
      </Link>
    </section>
  )
}

const cta = [
  "pointer-cursor",
  "transition-all",
  "rounded-lg",
  "text-sm",
  "font-medium",
  "border",
  "focus:outline-none",
  "focus:ring-1",
  "focus:ring-zinc-700",
  "focus:ring-offset-1",
  "dark:focus:ring-zinc-200",
  "dark:text-zinc-50",
  "px-2.5",
  "py-0.5",
  "border-neutral-darkest",
  "bg-neutral-darker",
  "text-neutral-lighter",
  "dark:border-neutral-lightest",
  "dark:bg-neutral-lightest",
  "dark:text-neutral-darker",
].join(" ")
