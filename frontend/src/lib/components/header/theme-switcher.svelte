<script lang="ts">
	import { Sun, Moon, Desktop, Mobile } from "radix-icons-svelte"
	import Menu from "$lib/menu/menu.svelte"
	import Items from "$lib/menu/items.svelte"
	import Item from "$lib/menu/item.svelte"
</script>

<Menu>
	<Sun slot="icon" />
	<Items classes="w-32">
		<form method="POST">
			<Item>
				<button
					formaction="/?/setTheme&theme=light"
					class="w-full flex flex-row gap-2 items-center"
				>
					<Sun class="text-stone-700" /><span>Light</span>
				</button>
			</Item>
			<Item>
				<button formaction="/?/setTheme&theme=dark" class="w-full flex flex-row gap-2 items-center">
					<Moon class="text-stone-700" /><span>Dark</span>
				</button>
			</Item>
			<Item>
				<button
					formaction="/?/setTheme&theme=system"
					class="w-full flex flex-row gap-2 items-center"
				>
					<div class="grid">
						<Desktop class="col-start-1 row-start-1 text-stone-700 collapse sm:visible" />
						<Mobile class="col-start-1 row-start-1 text-stone-700 visible sm:collapse " />
					</div>
					<span>System</span>
				</button>
			</Item>
		</form>
	</Items>
</Menu>
