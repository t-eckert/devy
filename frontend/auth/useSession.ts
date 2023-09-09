import { create } from "zustand"

import Session from "./Session"

interface SessionStore {
	session: Option<Session>
	setSession: (session: Session) => void
}

const useSession = create<SessionStore>((set) => ({
	session: null,
	setSession: (session) => set({ session }),
}))

export default useSession
