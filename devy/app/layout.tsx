import Link from "next/link"
import { Inter } from "next/font/google"

import Nav from "@/components/sections/Nav"

import "./globals.css"

const inter = Inter({ subsets: ["latin"] })

export const metadata = {
  title: "Devy",
  description: "Blog in Markdown from your GitHub repo",
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className={[inter.className, "text-slate-900"].join(" ")}>
        <header className="px-2 py-1 w-full flex flex-row items-center justify-between">
          <div className="flex flex-row gap-1.5 items-baseline">
            <h1 className="text-sm font-medium">
              <Link href="/">Devy</Link>
            </h1>
            <span className="px-1.5 py-0.25 text-xs rounded-full bg-slate-500 text-white font-medium">
              Pre-release
            </span>
          </div>
          <div className="flex flex-row">
            <button className="px-2 py-0.5 text-sm font-medium rounded-xl">
              Sign in with GitHub
            </button>
            <Nav />
          </div>
        </header>
        {children}
        <footer></footer>
      </body>
    </html>
  )
}
