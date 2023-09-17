"use client"

import { useSearchParams } from "next/navigation"

import useSession from "@/auth/useSession"

export default function CaptureSession() {
	const searchParams = useSearchParams()
	const session = useSession()

	if (session.status === "logged-out" && searchParams.has("token")) {
		const token = searchParams.get("token")

		console.log("loading session")
		session.loadSession(token)
	}

	return <></>
}
