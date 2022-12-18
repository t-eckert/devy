import type { NextApiRequest, NextApiResponse } from "next"
import POST from "./post"

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  switch (req.method) {
    case "POST":
      await POST(req, res)
    default:
      res.status(501).end()
  }
}
