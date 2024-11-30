import { setContext, getContext } from "svelte"

export type Theme = "light" | "dark"

class ThemeState {
  theme = $state<Theme>("light")

  constructor() {
    this.theme = localStorageTheme() || systemTheme() || "light"
  }

  toggle() {
    this.theme = this.theme === "light" ? "dark" : "light"
    setLocalStorageTheme(this.theme)
  }
}

function setLocalStorageTheme(theme: Theme) {
  if (typeof window === 'undefined') return
  localStorage.setItem("theme", theme)
}

function localStorageTheme(): Theme | undefined {
  if (typeof window === 'undefined') return undefined
  return localStorage.getItem("theme") as Theme
}

function systemTheme(): Theme | undefined {
  if (typeof window === 'undefined') return undefined
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

const THEME_KEY = Symbol("theme")

export function setTheme() {
  return setContext(THEME_KEY, new ThemeState())
}

export function getTheme() {
  return getContext<ReturnType<typeof setTheme>>(THEME_KEY)
}
