import { getContext, setContext } from "svelte"
import { parseSessionToken } from "$lib/auth"
import type { Session } from "$lib/types"

class User {
  #token = $state<string | null>(null)
  #session: Session | null = $derived(sessionOrNull(this.#token))

  isAuthenticated: boolean = $derived(this.#session !== null)

  userId = $derived(this.#session?.userId)
  profileId = $derived(this.#session?.profileId)
  username = $derived(this.#session?.username)
  role = $derived(this.#session?.role)
  status = $derived(this.#session?.status)
  displayName = $derived(this.#session?.displayName)
  avatarUrl = $derived(this.#session?.avatarUrl)

  constructor(token?: string) {
    if (token === undefined) {
      return
    }

    this.#token = token
  }

  setToken(token: string) {
    this.#token = token
  }

  unsetToken() {
    this.#token = null
  }
}

function sessionOrNull(token: string | null): Session | null {
  if (token === null) {
    return null
  }

  try {
    return parseSessionToken(token).body
  } catch {
    return null
  }
}

const SESSION_KEY = Symbol("user")

export function setUser(token?: string) {
  return setContext(SESSION_KEY, new User(token))
}

export function getUser() {
  return getContext<ReturnType<typeof setUser>>(SESSION_KEY)
}
