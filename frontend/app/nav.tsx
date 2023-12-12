"use client"

import NavToken from "./nav.token"
import NavMenu from "./nav.menu"
import NavThemeToggle from "./nav.theme-toggle"

export default function Nav() {
  return (
    <nav className="flex flex-row gap-1 items-center">
      <NavToken />
      <NavThemeToggle />
      <NavMenu />
    </nav>
  )
}

