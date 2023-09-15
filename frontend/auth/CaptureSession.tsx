"use client"

import { useSearchParams, useRouter } from "next/navigation"

import useSession from "@/auth/useSession"

export default function CaptureSession() {
	const searchParams = useSearchParams()
	const router = useRouter()
	const session = useSession()

	const token = searchParams.get("token")

	session.loadSession(token)
	router.replace("/")

	return <></>
}
