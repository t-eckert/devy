import "./globals.css"
import type { Metadata } from "next"
import { Inter } from "next/font/google"
import { CaptureSession } from "@/lib/auth"

import Header from "./header"
import Footer from "./footer"

const version = "v0.2.0"

const inter = Inter({ subsets: ["latin"] })

export const metadata: Metadata = {
  title: "Devy",
  description: "Markdown is the medium.",
}

interface Props {
  children: React.ReactNode
}

export default function RootLayout({ children }: Props) {
  return (
    <html lang="en" className="dark">
      <body
        className={[
          inter.className,
          "bg-neutral-light text-neutral-low dark:bg-neutral-low dark:text-neutral-light",
        ].join(" ")}
      >
        <CaptureSession />

        <div className="min-h-screen">
          <Header />

          <main>{children}</main>
        </div>

        <Footer version={version} />
      </body>
    </html>
  )
}
