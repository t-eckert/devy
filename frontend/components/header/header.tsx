import Home from "./home"
import Nav from "./nav"

export default function Header() {
	return (
		<header className="border-b border-b-neutral+1 dark:border-b-neutral-1">
			<div className="mx-auto max-w-6xl px-3 py-3 flex flex-row justify-between items-center">
				<Home />
				<Nav />
			</div>
		</header>
	)
}
