import Link from "@/components/link"
import Blog from "@/models/Blog"
import Profile from "@/models/Profile"
import User from "@/models/User"
import Image from "next/image"

interface Props {
	user: User
	profile: Profile
	blogs: Blog[]
}

export default function Member({ user, profile, blogs }: Props) {
	return (
		<section className="w-48 rounded-xl bg-zinc-950 border border-zinc-700 shadow p-2">
			<div className="flex flex-col items-start gap-3">
				<div>
					<Header
						avatarUrl={profile.avatarUrl}
						name={profile.displayName}
					/>
				</div>
				<div className="flex flex-col">
					<h1 className="font-semibold">{profile.displayName}</h1>
					{blogs &&
						blogs.map((blog, i) => (
							<Link
								href={`/${blog.slug}`}
								key={i}
								className="text-sm text-zinc-400 font-medium"
							>
								{blog.name}
							</Link>
						))}
				</div>
				<div>
					<Link
						href={`https://github.com/${user.githubUsername}`}
						className="text-sm text-zinc-400 font-medium"
					>
						GitHub
					</Link>
				</div>
				<div className="self-end">
					<div className="text-xs px-1 text-zinc-400 bg-zinc-900 rounded border border-zinc-500">
						{new Date(user.createdAt).toLocaleDateString()}
					</div>
				</div>
			</div>
		</section>
	)
}

const Header = ({
	avatarUrl: avatarUrl,
	name,
}: {
	avatarUrl?: string
	name: string
}) => {
	return (
		<>
			{avatarUrl ? (
				<Image
					src={avatarUrl}
					width={384}
					height={384}
					alt={`Profile of ${name}`}
					className="rounded-lg"
				/>
			) : (
				<div className="w-44 rounded-lg aspect-square bg-[radial-gradient(ellipse_at_bottom_left,_var(--tw-gradient-stops))] from-indigo-200 via-red-200 to-yellow-100" />
			)}
		</>
	)
}
