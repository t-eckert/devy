import { setContext, getContext } from "svelte"

export type Theme = "light" | "dark"

class ThemeState {
  constructor(theme: Theme) {
    this.theme = theme
  }

  theme = $state<Theme>("dark")

  toggle() {
    if (this.theme == "light") {
      this.theme = "dark"
    } else if (this.theme == "dark") {
      this.theme = "light"
    }
  }
}

const THEME_KEY = Symbol("theme")

export function setTheme(theme: Theme) {
  return setContext(THEME_KEY, new ThemeState(theme))
}

export function getTheme() {
  return getContext<ReturnType<typeof setTheme>>(THEME_KEY)
}
