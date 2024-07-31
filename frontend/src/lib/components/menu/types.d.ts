import type { SvelteComponentTyped } from "svelte"
import type MenuButton from "./menu.button.svelte"
import type MenuFormButton from "./menu.form-button.svelte"
import type MenuLink from "./menu.link.svelte"
import type MenuSep from "./menu.sep.svelte"

declare module "./menu.svelte" {
	export interface MenuType extends SvelteComponentTyped {
		Button: typeof MenuButton
		FormButton: typeof MenuFormButton
		Link: typeof MenuLink
		Sep: typeof MenuSep
	}

	const Menu: MenuType
	export default Menu
}
