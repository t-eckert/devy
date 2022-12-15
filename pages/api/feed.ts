import { PostMetadata } from "interfaces"
import type { NextApiRequest, NextApiResponse } from "next"
import path from "path"
import { promises as fs } from "fs"

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  const dir = path.join(process.cwd(), "fixtures")
  const posts = await fs.readFile(dir + "/feed.json", "utf8")

  res.status(200).json(JSON.parse(posts))
}
