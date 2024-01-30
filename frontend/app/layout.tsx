import "./globals.css"
import type { Metadata } from "next"
import { Inter } from "next/font/google"
import { Analytics } from "@vercel/analytics/react"
import { SpeedInsights } from "@vercel/speed-insights/next"

import { CaptureSession } from "@/lib/auth"
import QueryProvider from "@/lib/query-provider"

import Header from "@/components/header"
import Footer from "@/components/footer"
import ThemeProvider from "@/components/theme-provider"
import VersionAnnouncement from "@/components/version-announcement"

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
    <html lang="en" suppressHydrationWarning>
      <body>
        <SpeedInsights />
        <ThemeProvider>
          <QueryProvider>
            <div className={root}>
              <CaptureSession />
              <VersionAnnouncement />

              <div className="min-h-screen">
                <Header />

                <main>{children}</main>
              </div>

              <Footer />
            </div>
          </QueryProvider>
          <Analytics />
        </ThemeProvider>
      </body>
    </html>
  )
}

const root = [
  inter.className,
  "bg-neutral+3",
  "dark:bg-neutral-2",
  "text-neutral-2",
  "dark:text-neutral+2",
].join(" ")
