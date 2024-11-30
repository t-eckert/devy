import { setContext, getContext } from "svelte"

export type Theme = "light" | "dark" | "system"

class ThemeState {
  theme = $state<Theme>()

  constructor(theme?: Theme) {
    this.theme = theme
  }
}

const THEME_KEY = Symbol("theme")

export function setTheme(theme?: Theme) {
  return setContext(THEME_KEY, new ThemeState(theme))
}

export function getTheme() {
  return getContext<ReturnType<typeof setTheme>>(THEME_KEY)
}
