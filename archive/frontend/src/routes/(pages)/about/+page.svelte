<script lang="ts">
	import P from "$lib/elements/p.svelte"
	import Link from "$lib/elements/link.svelte"
</script>

<div
	class="w-full mx-auto flex sm:justify-center sm:items-center transition-all my-24 sm:my-36 xl:my-80 px-3"
>
	<div class="w-full max-w-2xl text-6xl sm:text-8xl flex flex-col gap-2 sm:gap-6">
		<span class="flex flex-row justify-start">
			<span>git,</span>
		</span>
		<span class="flex flex-row justify-center">
			<span>push,</span>
		</span>
		<span class="flex flex-row justify-end">
			<span>blog.</span>
		</span>
	</div>
</div>

<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3">
	<dir class="mx-auto w-full max-w-md">
		<p class="font-medium text-2xl mb-2">Devy makes it easy to share your thoughts.</p>
		<P
			>Write Markdown files and push them to a repository on GitHub. Link the repository to a blog
			on Devy and every <code class="bg-stone-200">git push</code> will publish the Markdown files as
			posts on your blog.</P
		>
		<P
			>Devy is an open source platform for sharing your learnings. Every blog post is shared to a
			common feed where any user can discover it.</P
		>
	</dir>

	<dir class="mx-auto w-full max-w-md">
		<p class="font-medium text-2xl mb-2">Create your own blogging workflow.</p>

		<P>Because Devy uses Markdown and Git, you can blog any way your want!</P><P
			>Create a CLI to publish your posts on a schedule. Use <Link href="https://obsidian.md/"
				>Obsidian</Link
			> to write your posts and auto sync to a repository. The possibilities are as endless as your imagination.</P
		>
	</dir>

	<dir class="mx-auto w-full max-w-md">
		<p class="font-medium text-2xl mb-2">And we're just getting started.</p>
		<P
			>Devy is new and evolving every day. Check out the <Link href="/changelog">changelog</Link>.
			All of the code is open source in the <Link href="https://github.com/t-eckert/devy"
				>Devy repo</Link
			>.</P
		>
		<P
			>Have feedback? <Link href="https://github.com/t-eckert/devy/issues/new/choose"
				>Open an issue</Link
			>.</P
		>
		<P
			>Want to talk blogging and development? <Link href="https://discord.gg/qUJ54EEC5g"
				>Join the Discord</Link
			>.</P
		>
	</dir>
</div>
