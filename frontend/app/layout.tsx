import "./globals.css"
import type { Metadata } from "next"
import { Inter } from "next/font/google"
import { Analytics } from "@vercel/analytics/react"

import { CaptureSession } from "@/lib/auth"
import QueryProvider from "@/lib/query-provider"

import Header from "./header"
import Footer from "./footer"
import ThemeProvider from "./theme.provider"

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
      <ThemeProvider>
        <body
          className={[
            inter.className,
            "bg-neutral+3 dark:bg-neutral-2",
          ].join(" ")}
        >
          <QueryProvider>
            <div className="bg-neutral-lighter text-neutral-darker dark:bg-neutral-darker dark:text-neutral-lighter">
              <CaptureSession />

              <div className="min-h-screen">
                <Header />

                <main>{children}</main>
              </div>

              <Footer />
            </div>
          </QueryProvider>
          <Analytics />
        </body>
      </ThemeProvider>
    </html>
  )
}
