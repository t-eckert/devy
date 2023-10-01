import Nav from "@/components/dynamic/Nav"
import "./globals.css"
import type { Metadata } from "next"
import { Inter } from "next/font/google"
import { Link } from "@/components/elements"
import CaptureSession from "@/auth/CaptureSession"

const version = "v0.2.0"

const inter = Inter({ subsets: ["latin"] })

export const metadata: Metadata = {
  title: "Devy",
  description: "Blog in markdown from your GitHub repo.",
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" className="dark">
      <body
        className={[
          inter.className,
          "bg-zinc-50 text-zinc-950 dark:bg-zinc-900 dark:text-zinc-50",
        ].join(" ")}
      >
        <CaptureSession />

        <div className="min-h-screen">
          <header className="mx-auto max-w-6xl px-2 py-3 flex flex-row justify-between items-center">
            <div className="flex flex-row gap-2 items-baseline">
              <Link href="/">
                <h1 className="font-semibold">Devy</h1>
              </Link>
            </div>

            <Nav />
          </header>

          <main>{children}</main>
        </div>

        <footer className="w-full bg-zinc-950">
          <div className="mx-auto max-w-6xl px-2 pt-8 pb-44">
            <div className="w-56 pb-2 border-b border-b-zinc-700 flex flex-row gap-2 items-baseline">
              <Link href="/">
                <h1 className="font-semibold">Devy</h1>
              </Link>
              <span className="text-xs rounded-full px-2 py-0.5 select-none bg-zinc-800">
                Pre-release {version}
              </span>
            </div>

            <div className="pt-2 flex flex-row gap-12 text-sm text-zinc-300">
              <section className="w-56 flex flex-col gap-4">
                <p>
                  Devy is an open source platform for people to publish blog
                  posts in markdown from their GitHub repositories.{" "}
                </p>
                <p>Thank you for checking it out.</p>
                <span className="self-end">
                  &#8212;{" "}
                  <Link href="https://thomaseckert.dev">Thomas Eckert</Link>
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
      </body>
    </html>
  )
}
