import Nav from "@/components/dynamic/Nav"
import "./globals.css"
import type { Metadata } from "next"
import { Inter } from "next/font/google"
import { Link } from "@/components/elements"
import CaptureSession from "@/auth/CaptureSession"

const inter = Inter({ subsets: ["latin"] })

export const metadata: Metadata = {
  title: "Devy",
  description: "Blog in markdown",
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
        <section className="mx-auto max-w-6xl px-2 py-3 flex flex-row justify-between items-center">
          <div className="flex flex-row gap-2 items-baseline">
            <Link href="/">
              <h1 className="font-semibold">Devy</h1>
            </Link>
            <span className="text-xs rounded-full px-2 py-0.5 select-none bg-zinc-800">
              Pre-release
            </span>
          </div>

          <Nav />
        </section>

        {children}
      </body>
    </html>
  )
}
