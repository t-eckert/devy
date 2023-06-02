import "./globals.css"
import { Inter } from "next/font/google"

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
      <body className={inter.className}>
        <header className="px-2 py-1 w-full flex flex-row items-center justify-between">
          <div className="flex flex-row gap-1.5 items-baseline">
            <h1 className="text-sm font-medium">Devy</h1>
            <span className="px-1 py-0.25 text-xs rounded-full border border-slate-700 font-medium">
              Pre-release
            </span>
          </div>
          <div>
            <button className="px-2 py-0.5 text-sm font-medium rounded-xl border border-slate-700">
              Sign in with GitHub
            </button>
          </div>
        </header>
        {children}
        <footer></footer>
      </body>
    </html>
  )
}
