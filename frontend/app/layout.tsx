import "./globals.css"
import type { Metadata } from "next"
import { Inter } from "next/font/google"
import { Analytics } from "@vercel/analytics/react"

import { CaptureSession } from "@/lib/auth"
import QueryProvider from "@/lib/query-provider"

import Header from "./header"
import Footer from "./footer"

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
    <html lang="en">
      <body className={inter.className}>
        <QueryProvider>
          <div id="dark-mode-wrapper" className="dark">
            <div className="bg-neutral-lighter text-neutral-darker dark:bg-neutral-darker dark:text-neutral-lighter">
              <CaptureSession />

              <div className="min-h-screen">
                <Header />

                <main>{children}</main>
              </div>

              <Footer />
            </div>
          </div>
        </QueryProvider>
        <Analytics />
      </body>
    </html>
  )
}
