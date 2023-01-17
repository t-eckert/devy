import Link from "components/Link"
import User from "lib/user"

export interface Props {
  user?: User
}

const Nav: React.FC<Props> = ({ user }) => {
  return (
    <nav className="mx-auto max-w-6xl px-2 py-1 flex flex-row justify-between">
      <div className="flex flex-row gap-2 items-center">
        <Link href="/" className="text-sm font-medium">
          Devy
        </Link>
        <div className="px-1.5 py-0.25 text-xs font-medium bg-yellow-300 text-yellow-900 rounded-full">
          ALPHA
        </div>
      </div>
      <div>
        {user ? (
          <Link href={`/${user?.name}`}>{user.name}</Link>
        ) : (
          <Link href="/login" style="button" className="text-sm">
            <span>Login with GitHub</span>
          </Link>
        )}
      </div>
    </nav>
  )
}

export default Nav
