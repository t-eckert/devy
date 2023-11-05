"use client"

import { useEffect, useState } from "react"

import { useQuery, useMutation } from "@tanstack/react-query"

import useSession from "@/lib/auth/useSession"
import useStore from "@/lib/useStore"
import api from "@/lib/api"
import { Like, Post } from "@/models"

import LikesLoggedIn from "./likes-logged-in"
import LikesLoggedOut from "./likes-logged-out"

interface Props {
	postId: string
	initialCount: number
}

export default function Likes({ postId, initialCount }: Props) {
	// Get the current user session.
	const session = useStore(useSession, (session) => session)

	// If no user is logged in, the likes component will be non-interactive.
	const hasUser = session?.status === "logged-in"

	// Set if the post is liked by the current user.
	const [likedByUser, setLikedByUser] = useState(false)
	const username = session?.session?.user?.username
	const { data: profileLikes } = useQuery({
		queryKey: ["likes", username],
		queryFn: () => {
			if (!username) return Promise.resolve([])
			return api.get<Post[]>(`/v1/profiles/${username}/likes`, 0)
		},
	})
	useEffect(() => {
		if (!hasUser || !profileLikes) {
			setLikedByUser(false)
			return
		}

		console.log(profileLikes)
		const isLiked = profileLikes.some((post) => post.id === postId) ?? false
		setLikedByUser(isLiked)
	}, [profileLikes, hasUser, postId])

	// Query the count of likes for the post and update the count state.
	const [count, setCount] = useState(initialCount)
	const { data: queriedCount } = useQuery({
		queryKey: ["likes", postId],
		queryFn: async () => {
			const post = await api.get<Post>(`/v1/posts/${postId}`, 0)
			return post.likes
		},
		initialData: initialCount,
	})
	useEffect(() => {
		setCount(queriedCount ?? initialCount)
	}, [queriedCount, initialCount])

	// Mutation to like or unlike the post.
	const { mutate: like } = useMutation({
		mutationFn: async () => {
			if (!hasUser) return Promise.resolve()
			if (likedByUser) {
				await api.delete(
					`/v1/likes/${postId}/${session?.session?.profile.id}`
				)
			} else {
				await api.post<Like>(`/v1/likes`, {
					postId,
					profileId: session?.session?.profile.id || "",
				})
			}
		},
		onSuccess: () => {
			setLikedByUser(!likedByUser)
			setCount(likedByUser ? count - 1 : count + 1)
		},
	})

	return hasUser ? (
		<LikesLoggedIn count={count} isLiked={likedByUser} onClick={like} />
	) : (
		<LikesLoggedOut count={count} />
	)
}
