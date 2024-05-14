<script lang="ts">
	let typeValue: string

	export let value: string = ""
	export let placeholder: string = ""
	export let optional: boolean = false
	export let description: string
	export let name: string
	export let prefix: string
	export let keyboardShortcut: string
	export let errorMessage: string

	let valid = true
	$: valid = !errorMessage
</script>

<div class="w-full flex flex-col gap-2 transition-all">
	<div class="relative">
		<div
			class="absolute -top-2 left-2 inline-block bg-white px-1 text-xs font-medium text-gray-900"
		>
			<label for={name}>{name}</label>
			{#if optional}
				<span class="text-gray-400"> (optional)</span>
			{/if}
		</div>
		<div
			class={[
				"flex rounded-md shadow-sm ring-1 ring-inset focus-within:ring-2 focus-within:ring-inset focus-within:ring-indigo-600 sm:max-w-md",
				valid ? "ring-gray-300" : "ring-red-400"
			].join(" ")}
		>
			{#if prefix}
				<span class="flex select-none items-center pl-3 text-gray-500 sm:text-sm">{prefix}</span>
			{/if}

			<input
				type={typeValue || "text"}
				{name}
				id={name}
				class="block w-full rounded-md px-2 py-1.5 text-gray-900 placeholder:text-gray-400 bg-transparent focus:outline-none sm:text-sm sm:leading-6"
				{placeholder}
				{value}
			/>

			{#if !valid}
				<div class="pointer-events-none flex items-center mr-1">
					<svg
						class="h-5 w-5 text-red-500"
						viewBox="0 0 20 20"
						fill="currentColor"
						aria-hidden="true"
					>
						<path
							fill-rule="evenodd"
							d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-5a.75.75 0 01.75.75v4.5a.75.75 0 01-1.5 0v-4.5A.75.75 0 0110 5zm0 10a1 1 0 100-2 1 1 0 000 2z"
							clip-rule="evenodd"
						/>
					</svg>
				</div>
			{/if}

			{#if keyboardShortcut}
				<div class="flex py-1.5 pr-1.5">
					<kbd
						class="inline-flex items-center rounded border border-gray-200 px-1 font-sans text-xs text-gray-400"
						>{keyboardShortcut}</kbd
					>
				</div>
			{/if}
		</div>
	</div>

	{#if errorMessage}
		<p class="text-xs text-red-500">{errorMessage}</p>
	{/if}

	{#if description}
		<p class="text-xs text-zinc-600">{description}</p>
	{/if}
</div>
