import Link from "components/Link"
import User from "lib/user"

export interface Props {
	user?: User
}

const Nav: React.FC<Props> = ({ user }) => {
	return (
		<nav className="flex flex-row justify-between">
			<Link href="/">Devy</Link>
			<div>
				{user ? (
					<Link href={`/${user?.name}`}>{user.name}</Link>
				) : (
					<Link href="/login">Login with GitHub</Link>
				)}
			</div>
		</nav>
	)
}

export default Nav
