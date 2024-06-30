<script lang="ts">
	import Main from "$lib/layouts/main.svelte"
	import Link from "$lib/elements/link.svelte"
	import { PUBLIC_API } from "$env/static/public"
</script>

<Main>
	<article class="mx-auto w-full max-w-2xl">
		<h1 class="text-2xl font-semibold mb-2">Documentation</h1>
		<p class="mb-2">
			This docs page exists to answer all of your questions about how to use Devy. If you are having
			issues using this preview version of Devy, <Link href="/feedback">please leave feedback</Link
			>. I read all of the submissions.
		</p>

		<h2 class="text-xl font-semibold mt-4 mb-2">Creating and posting to a blog</h2>
		<p class="mb-2">
			Once you have <Link href={PUBLIC_API + "/auth/login"}>signed in</Link>, you will need to <Link
				href="https://github.com/apps/devy-github">add Devy to your GitHub account</Link
			>. This enables Devy to receive webhooks when you push to your repository. Then, go to the
			<Link href="/new/blog">create new blog page</Link> and select a repository from the list*. Devy
			will automatically pull the repo and create a blog post for every markdown file in it.
		</p>
		<p class="mb-2">
			As you push to the default branch on that repository, Devy will pull your updates and publish
			them as a posts.
		</p>
		<span>*As of right now, Devy only lists your public repositories</span>
	</article>
</Main>
