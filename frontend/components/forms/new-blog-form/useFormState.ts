import { Dispatch, SetStateAction, useEffect, useState } from "react"
import { useRouter } from "next/navigation"

import api from "@/lib/api"

import GitHubRepo from "./GitHubRepo"
import { Blog, Repo } from "@/models"

type FormStatus =
	| "unsubmitted"
	| "submitting"
	| "successfully-submitted"
	| "failed-to-submit"

interface FormState {
	status: FormStatus
	error?: string

	fields: {
		blogName: string
		setBlogName: Dispatch<SetStateAction<string>>

		repoId: number
		setRepoId: Dispatch<SetStateAction<number>>
	}

	isComplete: boolean
	submit: () => void
}

export default function useFormState(
	username: string,
	repos: GitHubRepo[]
): FormState {
	const router = useRouter()

	const [status, setStatus] = useState<FormStatus>("unsubmitted")
	const [error, setError] = useState<string | undefined>()
	const [blogName, setBlogName] = useState<string>("")
	const [repoId, setRepoId] = useState<number>(-1)
	const [isComplete, setIsComplete] = useState<boolean>(false)

	useEffect(() => {
		if (blogName === "" || repoId === -1) {
			setIsComplete(false)
			return
		}

		setIsComplete(true)
	}, [blogName, repoId])

	const submit = async () => {
		if (!isComplete) {
			return
		}

		setStatus("submitting")

		const repo = repos.find((repo) => repo.id === repoId)
		if (!repo) {
			setStatus("failed-to-submit")
			setError("Repo not found")
			return
		}

		const data = {
			username,
			name: blogName,
			repoUrl: repo.html_url,
			githubId: repo.id,
			githubName: repo.name,
		}

		try {
			const res = await api.submit<{ blog: Blog; repo: Repo }>(
				"/v1/forms/new-blog",
				data
			)
			setStatus("successfully-submitted")
			router.push("/" + res.blog.slug)
		} catch (e) {
			setStatus("failed-to-submit")
			return
		}
	}

	return {
		status,
		error,

		fields: {
			blogName,
			setBlogName,

			repoId,
			setRepoId,
		},

		isComplete,
		submit,
	}
}
