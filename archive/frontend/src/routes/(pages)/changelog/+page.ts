import type { PageLoad } from "./$types"
import { error, type NumericRange } from "@sveltejs/kit"

export const load: PageLoad = async ({ url, fetch }) => {
  async function fetchIndex() {
    const resp = await fetch(`${url.origin}/changelog/index.json`, { headers: { accept: "application/json" } })
    if (!resp.ok) {
      throw error(resp.status as NumericRange<400, 599>, {
        message: resp.statusText
      })
    }

    let index: string[]
    try {
      index = await resp.json()
    } catch (e) {
      throw error(500, {
        message: e.message
      })
    }

    return index
  }

  async function fetchChangelog(version: string) {
    const resp = await fetch(`${url.origin}/changelog/${version}.md`, { headers: { accept: "text/markdown" } })
    if (!resp.ok) {
      throw error(resp.status as NumericRange<400, 599>, {
        message: resp.statusText
      })
    }

    let markdown: string
    try {
      markdown = await resp.text()
    } catch (e) {
      console.log(e.message)
      markdown = ""
    }
    return markdown
  }

  const index = await fetchIndex()
  return {
    changelogs: await Promise.all(index.map(async (version) => {
      const body = await fetchChangelog(version)
      return { version, body }
    }))
  }
}
