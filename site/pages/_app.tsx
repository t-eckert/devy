import "../styles/globals.scss"
import "../styles/markdown.scss"
import type { AppProps } from "next/app"
import { Inter } from "@next/font/google"
import Nav from "sections/Nav"

const inter = Inter({ subsets: ["latin"] })

export default function App({ Component, pageProps }: AppProps) {
  return (
    <div className={inter.className}>
      <Nav />
      <Component {...pageProps} />
    </div>
  )
}
