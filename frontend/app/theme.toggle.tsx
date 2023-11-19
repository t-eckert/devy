"use client"

import { useState, useEffect } from "react"
import { useTheme } from "next-themes"

export default function ThemeToggle() {
  const [mounted, setMounted] = useState(false)
  const { theme, setTheme } = useTheme()

  useEffect(() => {
    setMounted(true)
  }, [])

  if (!mounted) {
    return null
  }

  return (
    <div>
      The current theme is: {theme}
      <br />
      <button onClick={() => setTheme("light")}>Light Mode</button>
      <br />
      <button onClick={() => setTheme("dark")}>Dark Mode</button>
    </div>
  )
}
