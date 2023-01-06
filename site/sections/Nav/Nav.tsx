import Link from "components/Link"
import User from "lib/user"

export interface Props {
	user?: User
}

const Nav: React.FC<Props> = ({ user }) => {
	return (
		<nav className="px-2 py-1 flex flex-row justify-between">
			<Link href="/" className="text-sm font-medium">Devy</Link>
			<div>
				{user ? (
					<Link href={`/${user?.name}`}>{user.name}</Link>
				) : (
					<Link href="/login" style="button" className="text-sm"><span>Login with GitHub</span></Link>
				)}
			</div>
		</nav>
	)
}

export default Nav
