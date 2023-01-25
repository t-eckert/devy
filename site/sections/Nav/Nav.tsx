import Link from "components/Link"
import Breadcrumbs from "./Breadcrumbs"

import User from "lib/user"

export interface Props {
  path: string
  user?: User
}

const Nav: React.FC<Props> = ({ path, user }) => {
  return (
    <nav className="mx-auto max-w-6xl px-2 py-1 flex flex-row justify-between">
      <div className="flex flex-row gap-1 items-baseline select-none">
        <Link href="/" className="text-sm font-medium hover:underline">
          Devy
        </Link>
        <Breadcrumbs path={path} />
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
