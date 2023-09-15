import { create } from "zustand"
import jwt from "jsonwebtoken"

import Session from "./Session"

type SessionStatus = "logged-out" | "logged-in"

interface SessionStore {
	status: SessionStatus
	token: Option<string>
	session: Option<Session>
	loadSession: (token: Option<string>) => void
	clearSession: () => void
}

const useSession = create<SessionStore>((set) => ({
	status: "logged-out",
	token: null,
	session: null,
	loadSession: (token) =>
		set((state) => {
			if (!token || state.status === "logged-in") return state

			return {
				status: "logged-in",
				token,
				session: getSession(token),
			}
		}),
	clearSession: () => set(() => ({ status: "logged-out" })),
}))

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

export default useSession
