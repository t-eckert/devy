import Link from "@/components/link"
import Logomark from "@/components/logomark"

import Nav from "./nav"

export default function Header() {
  return (
    <header className="mx-auto max-w-6xl px-2 py-3 flex flex-row justify-between items-center">
      <div className="flex flex-row gap-2 items-baseline">
        <Link href="/" variant={{ underline: false }}>
          <Logomark />
        </Link>
        <span className="text-xs font-medium px-2 py-0.5 rounded-full bg-neutral-medium select-none">
          Preview
        </span>
      </div>

      <Nav />
    </header>
  )
}
