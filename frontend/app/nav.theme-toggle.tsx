
import { useState, useEffect } from "react"

import {
  MoonIcon,
  SunIcon,
} from "@radix-ui/react-icons"


import { useTheme } from "next-themes"
import Menu, { Item } from "@/components/menu"

export default function NavThemeToggle() {

  const [mounted, setMounted] = useState(false)
  const { theme, setTheme } = useTheme()

  useEffect(() => {
    setMounted(true)
  }, [])

  const themeIcon = theme === "light" ? <SunIcon /> : <MoonIcon />

  const themeMenuItems: Item[] = [
    {
      type: "button",
      label: "Light",
      onClick: () => {
        setTheme("light")
      },
    },
    {
      type: "button",
      label: "Dark",
      onClick: () => {
        setTheme("dark")
      },
    },
    {
      type: "button",
      label: "System",
      onClick: () => { },
    },
  ]

  if (!mounted) return null

  return <Menu
    icon={themeIcon}
    variant={{ hug: true }}
    ariaLabel="Theme selector"
    relation="below-aligned-left"
    items={themeMenuItems}
  />
}
