import Nav from "@/components/dynamic/Nav"
import "./globals.css"
import type { Metadata } from "next"
import { Inter } from "next/font/google"

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
    <html lang="en">
      <body className={[inter.className, "bg-zinc-900 text-white"].join(" ")}>
        <Nav />
        {children}
      </body>
    </html>
  )
}
