"use client"

import { useSearchParams, useRouter } from "next/navigation"
import jwt from "jsonwebtoken"

import useSession from "@/auth/useSession"
import Session from "./Session"

export default function CaptureSession() {
	const searchParams = useSearchParams()
	const router = useRouter()
	const { session, setSession } = useSession()

	const token = searchParams.get("token")

	if (token && !session) {
		const newSession = getSession(token)
		if (newSession) {
			setSession(newSession)
			router.replace("/")
		}
	}

	return <></>
}

const getSession = (token: string): Option<Session> => {
	const decoded = jwt.decode(token)

	if (!decoded) return null

	switch (typeof decoded) {
		case "string":
			try {
				return JSON.parse(decoded) as Session
			} catch (e) {
				return null
			}
		case "object":
			return decoded as Session
		default:
			return null
	}
}
