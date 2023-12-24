import Link from "@/components/link"

export default function CreateLoggedOut() {
  return (
    <div>
      <p className="mb-2 w-full max-w-sm">
        You need to sign in with your GitHub account before you can create a
        blog.
      </p>

      <Link
        href="/api/auth/login"
        prefetch={false}
        variant={{ underline: true, styled: true }}
        className="rounded-full focus:outline-none focus:ring-1 focus:ring-neutral-2 focus:dark:ring-neutral+2"
      >
        Sign in with GitHub
      </Link>
    </div>
  )
}
