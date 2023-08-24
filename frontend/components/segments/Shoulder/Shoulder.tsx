import { Link } from "@/components/elements"

export default function Shoulder() {
	return (
		<section className="flex flex-col w-64 gap-2">
			<div>
				<Link href="/changelog" variant={{ underline: true }}>
					Changelog
				</Link>
			</div>
		</section>
	)
}
