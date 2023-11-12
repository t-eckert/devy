import Link from "@/components/link"
import Logomark from "@/components/logomark"

export default function Footer() {
  return (
    <footer className="w-full dark:bg-neutral-darkest">
      <div className="mx-auto max-w-6xl px-2 pt-8 pb-44">
        <div className="w-56 pb-2 border-b border-b-zinc-700 flex flex-row gap-2 items-baseline">
          <Link href="/" variant={{ underline: false }}>
            <Logomark />
          </Link>
          <span className="text-xs font-medium px-2 py-0.5 rounded-full bg-neutral-medium select-none">
            Preview
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
              &#8212;{" "}
              <Link
                href="https://thomaseckert.dev"
                variant={{ underline: false }}
              >
                Thomas Eckert
              </Link>
            </span>
          </section>

          <section className="flex flex-col gap-1 items-start">
            <Link
              href="/changelog"
              className="font-medium"
              variant={{ underline: false }}
            >
              Changelog
            </Link>
            <Link
              href="https://github.com/t-eckert/devy"
              className="font-medium"
              variant={{ underline: false }}
            >
              Repo
            </Link>
          </section>
        </div>
      </div>
    </footer>
  )
}
