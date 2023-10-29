"use client"

import { useSearchParams } from "next/navigation"

import useStore from "@/lib/useStore"
import useSession from "./useSession"

// This component is loaded at the root of the site in `layout`. It checks if
// there is a token in the URL and if so, loads the session.
export default function CaptureSession() {
	const searchParams = useSearchParams()
	const session = useStore(useSession, (state) => state)

	if (session?.status === "logged-out" && searchParams.has("token")) {
		const token = searchParams.get("token")

		console.log("loading session")
		session.loadSession(token)
	}

	return <></>
}
