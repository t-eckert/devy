import { create } from "zustand"
import { persist } from "zustand/middleware"
import jwt from "jsonwebtoken"

import Session from "./Session"

export type SessionStatus = "logged-out" | "logged-in"

export interface SessionStore {
	status: SessionStatus
	token?: string
	session?: Session
	loadSession: (token: Option<string>) => void
	clearSession: () => void
}

const useSession = create<SessionStore>()(
	persist(
		(set) => ({
			status: "logged-out",
			token: undefined,
			session: undefined,
			loadSession: (token) =>
				set((state) => {
					if (!token || state.status === "logged-in") return state

					return {
						status: "logged-in",
						token,
						session: decode(token),
					}
				}),
			clearSession: () =>
				set(() => ({
					status: "logged-out",
					token: undefined,
					session: undefined,
				})),
		}),
		{ name: "devy-session" }
	)
)

const decode = (token: string): Session | undefined => {
	const decoded = jwt.decode(token)

	if (!decoded) return undefined

	switch (typeof decoded) {
		case "string":
			try {
				return JSON.parse(decoded) as Session
			} catch (e) {
				return undefined
			}
		case "object":
			return decoded as Session
		default:
			return undefined
	}
}

export default useSession
