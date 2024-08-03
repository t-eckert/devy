import type { Session, TokenPayload } from "$lib/types"
import { jwtDecode } from "jwt-decode"

export function parseSessionToken(token: string): TokenPayload<Session> {
	return jwtDecode<TokenPayload<Session>>(token)
}
