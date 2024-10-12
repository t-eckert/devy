import { getContext, setContext } from "svelte"
import { sessionOrNull } from "$lib/auth"
import type { Session } from "$lib/types"

class User {
  private token = $state<string>()
  private session: Session | null = $derived(sessionOrNull(this.token))

  isAuthenticated: boolean = $derived(this.session !== null)

  userId = $derived(this.session?.userId)
  profileId = $derived(this.session?.profileId)
  username = $derived(this.session?.username)
  role = $derived(this.session?.role)
  status = $derived(this.session?.status)
  displayName = $derived(this.session?.displayName)
  avatarUrl = $derived(this.session?.avatarUrl)


  constructor(token?: string) {
    if (token === undefined) {
      return
    }

    this.token = token
  }

  setToken(token: string) {
    this.token = token
  }

  unsetToken() {
    this.token = undefined
  }
}

const SESSION_KEY = Symbol("user")

export function setUser(token?: string) {
  return setContext(SESSION_KEY, new User(token))
}

export function getUser() {
  return getContext<ReturnType<typeof setUser>>(SESSION_KEY)
}
