"use client"

import {
	useQuery,
	QueryClientProvider,
	QueryClient,
} from "@tanstack/react-query"
import { ReactQueryDevtools } from "@tanstack/react-query-devtools"

import { Post, Like } from "@/models"
import Preview from "@/components/post-preview"
import api from "@/lib/api"
import useStore from "@/lib/useStore"
import useSession from "@/lib/auth"

const queryClient = new QueryClient()

interface Props {
	posts: Post[]
}

const fetchLikes = async (username?: string) => {
	if (!username) return new Set<string>()

	const likes = await api.get<string[]>(
		`/v1/profiles/${username}/likes/ids`,
		10
	)
	return new Set(likes)
}

function Posts({ posts }: Props) {
	const state = useStore(useSession, (state) => state)

	const status = state?.status
	const session = state?.session

	const { data: userLikes } = useQuery({
		queryKey: [session?.user.username || "no-user"],
		queryFn: () => fetchLikes(session?.user.username),
	})

	const onLike = (postId: string) => {
		const profileId = session?.profile.id

		if (!profileId) return

		api.post<Like>("/v1/likes", {
			postId,
			profileId,
		}).then((res) => {
			return res
		})
	}

	const onUnlike = (postId: string) => {
		const profileId = session?.profile.id

		if (!profileId) return

		api.delete<Like>(`/v1/likes/${postId}/${profileId}`).then((res) => {
			return res
		})
	}

	return (
		<>
			{posts.map((post) => (
				<Preview key={post.id} post={post} />
			))}
		</>
	)
}

const PostsWithProvider = (props: Props) => {
	return (
		<QueryClientProvider client={queryClient}>
			<Posts {...props} />
			<ReactQueryDevtools initialIsOpen={false} />
		</QueryClientProvider>
	)
}

export default PostsWithProvider
