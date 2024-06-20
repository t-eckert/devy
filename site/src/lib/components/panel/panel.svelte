<section
	class="w-full rounded-md backdrop-blur-3xl shadow bg-white/60 dark:bg-zinc-900 dark:border-stone-200"
>
	<slot name="header" />
	<div class="p-3 border-x border-stone-200/70">
		<slot />
	</div>
	<slot name="footer" />
</section>
