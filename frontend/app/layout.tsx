import "./globals.css"
import type { Metadata } from "next"
import { Inter } from "next/font/google"

import { CaptureSession } from "@/lib/auth"
import QueryProvider from "@/lib/query-provider"

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
    <html lang="en">
      <QueryProvider>
        <div className="dark">
          <body
            className={[
              inter.className,
              "bg-neutral-lighter text-neutral-darker dark:bg-neutral-darker dark:text-neutral-lighter",
            ].join(" ")}
          >
            <CaptureSession />

            <div className="min-h-screen">
              <Header />

              <main>{children}</main>
            </div>

            <Footer version={version} />
          </body>
        </div>
      </QueryProvider>
    </html>
  )
}
