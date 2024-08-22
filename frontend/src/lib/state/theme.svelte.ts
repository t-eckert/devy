import { setContext, getContext } from "svelte"

export type Theme = "light" | "dark"

class ThemeState {
	constructor(theme: Theme) {
		this.#theme = theme
	}

	#theme = $state<Theme>("dark")

	toggle() {
		if (this.#theme == "light") {
			this.#theme = "dark"
		} else if (this.#theme == "dark") {
			this.#theme = "light"
		}
	}

	get theme() {
		return this.#theme
	}
}

const THEME_KEY = Symbol("theme")

export function setThemeState(theme: Theme) {
	return setContext(THEME_KEY, new ThemeState(theme))
}

export function getThemeState() {
	return getContext<ReturnType<typeof setThemeState>>(THEME_KEY)
}
