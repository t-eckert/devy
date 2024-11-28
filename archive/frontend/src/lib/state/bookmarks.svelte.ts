import type { Session } from "$lib/types"
import { setContext, getContext } from "svelte"
import { PUBLIC_API } from "$env/static/public"
import { sessionOrNull } from "$lib/auth"

// This contains the set of all bookmarks by the current user.
// It exposes methods to add and remove likes that optimistically update the state
// and send the request to the server.
class BookmarksState {
  constructor(token?: string) {
    this.token = token
  }

  private token = $state<string>()
  private session: Session | null = $derived(sessionOrNull(this.token))
  private userBookmarks = $state<string[]>([])

  // Sets the token ans session to authenticate requests as the current user.
  setToken(token: string) {
    this.token = token
  }

  // Loads the bookmarks for the current user.
  // Requires the token to be set. If not, it returns a resolved promise immediately.
  async loadBookmarks() {
    if (!this.token) {
      return Promise.resolve()
    }

    const response = await fetch(`${PUBLIC_API}/bookmarks/${this.session?.username}`, {
      headers: {
        Authorization: `Bearer ${this.token}`,
        "Content-Type": "application/json"
      }
    })
    if (response.ok) {
      const data = await response.json()
      this.userBookmarks = data.map(({ postId }: { postId: string }) => postId)
    }
  }

  // Returns true if the current user has bookmarked the post with the given ID.
  isBookmarked(postId: string): boolean {
    return this.userBookmarks.includes(postId)
  }

  // Adds a bookmark for the current user to the post with the given ID.
  async bookmark(postId: string) {
    this.userBookmarks.push(postId)

    const response = await fetch(`${PUBLIC_API}/bookmarks`, {
      headers: {
        Authorization: `Bearer ${this.token}`,
        "Content-Type": "application/json"
      },
      method: "POST",
      body: JSON.stringify({ profileId: this.session?.profileId, postId })
    })

    if (!response.ok) {
      this.userBookmarks = this.userBookmarks.filter((id) => id !== postId)
    }
  }

  // Removes a bookmark for the current user from the post with the given ID.
  async unbookmark(postId: string) {
    this.userBookmarks = this.userBookmarks.filter((id) => id !== postId)

    const response = await fetch(`${PUBLIC_API}/bookmarks/${this.session?.profileId}/${postId}`, {
      headers: {
        Authorization: `Bearer ${this.token}`,
        "Content-Type": "application/json"
      },
      method: "DELETE"
    })

    if (!response.ok) {
      this.userBookmarks.push(postId)
    }
  }

  // Toggles the like state for the current user on the post with the given ID.
  async toggle(postId: string) {
    if (this.isBookmarked(postId)) {
      await this.unbookmark(postId)
    } else {
      await this.bookmark(postId)
    }
  }

  // For debugging, returns a debug object with the current state.
  __debug() {
    return {
      userLikes: this.userBookmarks,
      token: this.token,
      session: this.session
    }
  }
}

const BOOKMARKS_KEY = Symbol("bookmarks")

export function setBookmarks(token?: string) {
  return setContext(BOOKMARKS_KEY, new BookmarksState(token))
}

export function getBookmarks() {
  return getContext<ReturnType<typeof setBookmarks>>(BOOKMARKS_KEY)
}
