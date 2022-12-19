import Link from "components/Link"

export const Header = () => {
	return (
		<header>
			<h1 className="text-3xl font-medium">Divy</h1>
			<p className="w-44 leading-tight">
				Blog in Markdown from your own GitHub repo.
			</p>
			<div className="flex flex-row gap-2">
				<Link href="/about">Learn more</Link>
				<Link href="/sign-in">Sign in with GitHub</Link>
			</div>
		</header>
	)
}
