import { marked } from "marked"

const reHeader = /^(#+)\s(.+)$/gm

export interface Header {
  id: string
  value: string
  level: number
}

export default function parse(markdown: string): string {
  return marked.parse(markdown)
}

export const headers = (markdown: string): Header[] => Array.from(markdown.matchAll(reHeader))
  .map((match) => ({
    id: match[2].replaceAll(" ", "-").replaceAll(/[^a-zA-Z0-9-]+/g, "").toLowerCase(),
    value: match[2],
    level: match[1].length,
  }))
