"use client"

import { useState, useEffect } from "react"

import {
  GitHubLogoIcon,
  HamburgerMenuIcon,
  MoonIcon,
  SunIcon,
} from "@radix-ui/react-icons"

import Link from "@/components/link"
import Avatar from "@/components/avatar"

import { useTheme } from "next-themes"
import Menu, { Item } from "@/components/menu"
import useSession from "@/lib/auth/useSession"
import type Session from "@/lib/auth/Session"
import useStore from "@/lib/useStore"
import config from "@/config"

import NavToken from "./nav.token"
import NavMenu from "./nav.menu"

export default function Nav() {
  const session = useStore(useSession, (state) => state)
  const [mounted, setMounted] = useState(false)
  const { theme, setTheme } = useTheme()

  useEffect(() => {
    setMounted(true)
  }, [])

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


  const themeIcon = theme === "light" ? <SunIcon /> : <MoonIcon />

  return (
    <nav className="flex flex-row gap-1 items-center">
      <NavToken />
      {mounted && (
        <Menu
          icon={themeIcon}
          variant={{ hug: true }}
          ariaLabel="Theme selector"
          relation="below-aligned-left"
          items={themeMenuItems}
        />
      )}
      <NavMenu />
    </nav>
  )
}

