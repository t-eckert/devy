import Link from "components/Link"

export interface Props {
	path: string
}

const Breadcrumbs: React.FC<Props> = ({ path }) => {
	const crumbs = path.split("/").filter((crumb) => crumb !== "")

	return (
		<span className="text-sm font-medium flex flex-row gap-1">
			{crumbs.map((crumb, i) => {
				const href = `/${crumbs.slice(0, i + 1).join("/")}`
				return (
					<span key={href}>
						<span className="mr-1 cursor-default">{"/"}</span>
						<Link href={href} className="hover:underline">
							{crumb}
						</Link>
					</span>
				)
			})}
		</span>
	)
}

export default Breadcrumbs
