import type { Session, TokenPayload } from "$lib/types"
import { jwtDecode } from "jwt-decode"

export function parseSessionToken(token: string): TokenPayload<Session> {
  return jwtDecode<TokenPayload<Session>>(token)
}

export function sessionOrNull(token?: string): Session | null {
  if (!token) {
    return null
  }

  try {
    return parseSessionToken(token).body
  } catch {
    return null
  }
}
