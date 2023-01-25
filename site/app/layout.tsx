import "../styles/globals.scss"
import "../styles/markdown.scss"
import { Inter } from "@next/font/google"
import Nav from "sections/Nav"

const inter = Inter({ subsets: ["latin"] })

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html>
      <head />
      <body className={inter.className}>
        <Nav />
        <div>{children}</div>
      </body>
    </html>
  )
}
