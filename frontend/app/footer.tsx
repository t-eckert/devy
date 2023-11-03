import Link from "@/components/link"
import Logomark from "@/components/logomark"

interface Props {
  version: string
}

export default function Footer({ version }: Props) {
  return (
    <footer className="w-full dark:bg-neutral-darkest">
      <div className="mx-auto max-w-6xl px-2 pt-8 pb-44">
        <div className="w-56 pb-2 border-b border-b-zinc-700 flex flex-row gap-2 items-baseline">
          <Link href="/">
            <Logomark />
          </Link>
          <span className="text-xs rounded-full px-2 py-0.5 select-none bg-zinc-800">
            Pre-release {version}
          </span>
        </div>

        <div className="pt-2 flex flex-row gap-12 text-sm text-zinc-300">
          <section className="w-56 flex flex-col gap-4">
            <p>
              Devy is an open source platform for people to publish blog posts
              in markdown from their GitHub repositories.{" "}
            </p>
            <p>Thank you for checking it out.</p>
            <span className="self-end">
              &#8212; <Link href="https://thomaseckert.dev">Thomas Eckert</Link>
            </span>
          </section>

          <section className="flex flex-col gap-1 items-start">
            <Link href="/changelog" className="font-medium">
              Changelog
            </Link>
            <Link
              href="https://github.com/t-eckert/devy"
              className="font-medium"
            >
              Repo
            </Link>
          </section>
        </div>
      </div>
    </footer>
  )
}
