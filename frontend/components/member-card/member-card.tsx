import Image from "next/image"

import Link from "@/components/link"

import { Blog, Profile, User } from "@/models"

interface Props {
	user: User
	profile: Profile
	blogs: Blog[]
}

export default function MemberCard({ user, profile, blogs }: Props) {
	return (
		<section className="w-56 flex flex-col rounded-xl shadow text-neutral-1 dark:text-neutral+1 transition-all">
			<MemberImage src={profile.avatarUrl} />

			<div className="p-2 flex flex-col items-start gap-2 rounded-b-xl bg-neutral+3 dark:bg-neutral-3 border-b border-x border-neutral+2 dark:border-neutral-2">
				<div className="flex flex-wrap items-baseline gap-2">
					<MemberName
						name={profile.displayName}
						username={user.username}
					/>
					<MemberSponsor />
				</div>
				<MemberWebsite website={profile.website} />
				<MemberBlogs blogs={blogs} />

				<div className="mt-4 w-full text-xs flex flex-row justify-between items-baseline">
					<MemberGitHub username={user.githubUsername} />
					<MemberJoined date={user.createdAt} />
				</div>
			</div>
		</section>
	)
}

const MemberImage = ({ src }: { src?: string }) => {
	if (!src) {
		return null
	}

	return (
		<Image
			src={src}
			width={224}
			height={224}
			alt="Avatar"
			className="rounded-t-xl object-cover aspect-square"
		/>
	)
}

const MemberName = ({
	name,
	username,
}: {
	name?: string
	username?: string
}) => {
	if (!name && !username) {
		return null
	}

	return (
		<Link
			href={{ pathname: `/profiles/${username}` }}
			className="font-medium dark:text-neutral+2 hover:dark:text-neutral+3"
			variant={{ underline: false }}
		>
			{name || username}
		</Link>
	)
}

const MemberSponsor = ({ isSponsor }: { isSponsor?: boolean }) => {
	if (!isSponsor) {
		return null
	}

	return (
		<div className="text-xs text-neutral-1 dark:text-neutral+1">
			<span className="px-1 py-0.5 rounded-md bg-neutral+2 dark:bg-neutral-2">
				Sponsor
			</span>
		</div>
	)
}

const MemberWebsite = ({ website }: { website?: string }) => {
	if (!website) {
		return null
	}

	return (
		<Link
			href={website}
			variant={{ underline: false }}
			className="font-medium text-sm text-neutral-1 hover:text-neutral-3 dark:text-neutral+1 hover:dark:text-neutral+3"
		>
			Website
		</Link>
	)
}

const MemberBlogs = ({ blogs }: { blogs: Blog[] }) => {
	if (blogs.length === 0) {
		return null
	}

	return (
		<div>
			<h2 className="text-xs text-neutral">
				Blog{blogs.length > 1 ? "s" : ""}
			</h2>
			<div className="flex flex-col gap-0.5">
				{blogs.map((blog) => (
					<Link
						key={blog.slug}
						href={{ pathname: `/${blog.slug}` }}
						variant={{ underline: false }}
						className="text-sm font-medium dark:text-neutral+2 hover:dark:text-neutral+3"
					>
						{blog.name}
					</Link>
				))}
			</div>
		</div>
	)
}

const MemberGitHub = ({ username }: { username?: string }) => {
	if (!username) {
		return null
	}

	return (
		<Link
			href={`https://github.com/${username}`}
			variant={{ underline: false }}
			className="dark:text-neutral+2 hover:dark:text-neutral+3"
		>
			GitHub
		</Link>
	)
}

const MemberJoined = ({ date }: { date: string }) => {
	const formatDate = (date: string) => {
		const d = new Date(date)
		return `${d.getFullYear()}.${d.getMonth() + 1}.${d.getDate()}`
	}
	return (
		<span className="px-1 py-0.5 rounded-md bg-neutral+2 dark:bg-neutral-2">
			{formatDate(date)}
		</span>
	)
}
