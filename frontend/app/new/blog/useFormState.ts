import { Dispatch, SetStateAction, useEffect, useState } from "react"

import api from "@/lib/api"

import GitHubRepo from "./GitHubRepo"

type FormStatus =
	| "unsubmitted"
	| "submitting"
	| "successfully-submitted"
	| "failed-to-submit"

interface FormState {
	status: FormStatus

	blogName: string
	setBlogName: Dispatch<SetStateAction<string>>

	repo: GitHubRepo | null
	setRepo: Dispatch<SetStateAction<GitHubRepo | null>>

	isComplete: boolean
	submit: () => void
}

export default function useFormState(): FormState {
	const [status, setStatus] = useState<FormStatus>("unsubmitted")
	const [blogName, setBlogName] = useState<string>("")
	const [repo, setRepo] = useState<GitHubRepo | null>(null)
	const [isComplete, setIsComplete] = useState<boolean>(false)

	useEffect(() => {
		if (!blogName || !repo) {
			setIsComplete(false)
			return
		}

		setIsComplete(true)
	}, [blogName, repo])

	return {
		status,

		blogName,
		setBlogName,

		repo,
		setRepo,

		isComplete,
		submit: () => {
			if (!isComplete) {
				return
			}

			setStatus("submitting")
		},
	}
}
