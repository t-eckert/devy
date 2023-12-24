import Link from "@/components/link"
import Logomark from "@/components/logomark"

export default function Footer() {
  return (
    <footer className="w-full bg-neutral+2 dark:bg-neutral-3 transition-all">
      <div className="mx-auto max-w-6xl px-3 pt-8 pb-44">
        <div className="w-full sm:w-56 pb-2 border-b border-b-neutral+1 dark:border-b-neutral-1 flex flex-row gap-2 items-baseline">
          <Link href="/" variant={{ underline: false, styled: false }}>
            <Logomark />
          </Link>
          <span className="text-xs font-medium px-2 py-0.5 rounded-full text-neutral-1 bg-neutral+1 dark:text-neutral+2 dark:bg-neutral-1 select-none">
            Preview
          </span>
        </div>

        <div className="pt-2 flex flex-col sm:flex-row gap-12 text-sm text-zinc-300">
          <section className="w-full sm:w-56 flex flex-col gap-4">
            <p>
              Devy is an open source platform for people to publish blog posts
              in markdown from their GitHub repositories.{" "}
            </p>
            <p>Thank you for checking it out.</p>
            <span className="self-end">
              &#8212;{" "}
              <Link
                href="https://thomaseckert.dev"
                variant={{ underline: true, styled: false }}
              >
                Thomas Eckert
              </Link>
            </span>
          </section>

          <section className="flex flex-row font-medium sm:flex-col gap-4 sm:gap-1 items-start">
            <Link href="/changelog" variant={{ styled: false }}>
              Changelog
            </Link>
            <Link
              href="https://github.com/t-eckert/devy"
              variant={{ styled: false }}
            >
              Repo
            </Link>
            <Link
              href="https://github.com/t-eckert/devy/issues/new/choose"
              variant={{ styled: false }}
            >
              Open an issue
            </Link>
          </section>
        </div>
      </div>
    </footer>
  )
}
