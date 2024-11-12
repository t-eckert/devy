import type { PageServerLoad } from "./$types";
import { API } from "$env/static/private";
import type { User, Blog, Profile, Entry } from "$lib/types"
import { error } from "@sveltejs/kit"

export const load: PageServerLoad = async ({ fetch, params }) => {
  async function fetchUser(): Promise<User> {
    const userResp = await fetch(`${API}/users/${params.username}`)
    if (!userResp.ok) {
      throw error(userResp.status, "User not found")
    }
    return await userResp.json()
  }

  async function fetchProfile(): Promise<Profile> {
    const profileResp = await fetch(`${API}/profiles/${params.username}`)
    if (!profileResp.ok) {
      throw error(profileResp.status, "Profile not found")
    }
    return await profileResp.json()
  }

  async function fetchBlogs(): Promise<Blog[]> {
    const blogsResp = await fetch(`/api/profiles/${params.username}/blogs`)
    if (!blogsResp.ok) {
      throw error(blogsResp.status, "Blogs not found")
    }
    return await blogsResp.json()
  }

  async function fetchEntries(): Promise<Entry[]> {
    const entriesResp = await fetch(`/api/profiles/${params.username}/entries`)
    if (!entriesResp.ok) {
      throw error(entriesResp.status, "Entries not found")
    }
    return await entriesResp.json()
  }

  return {
    user: await fetchUser(),
    profile: await fetchProfile(),
    blogs: await fetchBlogs(),
    entries: await fetchEntries()
  }
}
