import type { Session } from "$lib/types"
import { setContext, getContext } from "svelte"
import { sessionOrNull } from "$lib/auth"

// This contains the set of all likes by the current user.
// It exposes methods to add and remove likes that optimistically update the state
// and send the request to the server.
class LikesState {
  constructor(token?: string) {
    this.token = token
  }

  private token = $state<string>()
  private session: Session | null = $derived(sessionOrNull(this.token))
  private userLikes = $state<string[]>([])
  private likesCount = $state<{ postId: string; count: number }[]>([])

  // Sets the token ans session to authenticate requests as the current user.
  setToken(token: string) {
    this.token = token
  }

  // Loads the likes for the current user.
  // Requires the token to be set. If not, it returns a resolved promise immediately.
  async loadLikes() {
    if (!this.token) {
      return Promise.resolve()
    }

    const response = await fetch(`/api/likes/${this.session?.username}`, {
      headers: {
        Authorization: `Bearer ${this.token}`,
        "Content-Type": "application/json"
      }
    })
    if (response.ok) {
      const data = await response.json()
      this.userLikes = data.map(({ postId }: { postId: string }) => postId)
    }
  }

  // Sets the count of likes for a given post by its ID.
  setCount(postId: string, count: number) {
    this.likesCount = this.likesCount.filter((like) => like.postId !== postId)
    this.likesCount.push({ postId, count })
  }

  // Gets the count of likes for a given post by its ID.
  getCount(postId: string): number {
    return this.likesCount.find((like) => like.postId === postId)?.count || 0
  }

  // Returns true if the current user has liked the post with the given ID.
  isLiked(postId: string): boolean {
    return this.userLikes.includes(postId)
  }

  // Adds a like for the current user to the post with the given ID.
  async like(postId: string) {
    this.userLikes.push(postId)
    this.likesCount = this.likesCount.map((like) => {
      if (like.postId === postId) {
        return { postId, count: floorZero(like.count + 1) }
      }
      return like
    })

    const response = await fetch(`/api/likes`, {
      headers: {
        Authorization: `Bearer ${this.token}`,
        "Content-Type": "application/json"
      },
      method: "POST",
      body: JSON.stringify({ profileId: this.session?.profileId, postId })
    })

    if (!response.ok) {
      this.userLikes = this.userLikes.filter((id) => id !== postId)
      this.likesCount = this.likesCount.map((like) => {
        if (like.postId === postId) {
          return { postId, count: floorZero(like.count - 1) }
        }
        return like
      })
    }
  }

  // Removes a like for the current user from the post with the given ID.
  async unlike(postId: string) {
    this.userLikes = this.userLikes.filter((id) => id !== postId)
    this.likesCount = this.likesCount.map((like) => {
      if (like.postId === postId) {
        return { postId, count: floorZero(like.count - 1) }
      }
      return like
    })

    const response = await fetch(`/api/likes/${this.session?.username}/${postId}`, {
      headers: {
        Authorization: `Bearer ${this.token}`,
        "Content-Type": "application/json"
      },
      method: "DELETE"
    })

    if (!response.ok) {
      this.userLikes.push(postId)
      this.likesCount = this.likesCount.map((like) => {
        if (like.postId === postId) {
          return { postId, count: floorZero(like.count + 1) }
        }
        return like
      })
    }
  }

  // Toggles the like state for the current user on the post with the given ID.
  async toggle(postId: string) {
    if (this.isLiked(postId)) {
      await this.unlike(postId)
    } else {
      await this.like(postId)
    }
  }

  // For debugging, returns a debug object with the current state.
  __debug() {
    return {
      userLikes: this.userLikes,
      likesCount: this.likesCount,
      token: this.token,
      session: this.session
    }
  }
}

const LIKES_KEY = Symbol("likes")

export function setLikes(token?: string) {
  return setContext(LIKES_KEY, new LikesState(token))
}

export function getLikes() {
  return getContext<ReturnType<typeof setLikes>>(LIKES_KEY)
}

function floorZero(a: number): number {
  if (a < 0) {
    return 0
  }
  return a
}
