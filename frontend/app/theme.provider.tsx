"use client"
import { ThemeProvider } from "next-themes"
import { useState, useEffect } from "react"

interface Props {
  children: React.ReactNode
}

export default function ThemesProvider({ children }: Props) {
  const [mounted, setMounted] = useState(false)

  useEffect(() => {
    setMounted(true)
  }, [])

  if (!mounted) {
    return <>{children}</>
  }

  return <ThemeProvider attribute="dark">{children}</ThemeProvider>
}
