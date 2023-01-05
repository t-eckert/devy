import type { NextApiRequest, NextApiResponse } from "next"
import Upload from "lib/upload"

export default async function POST(req: NextApiRequest, res: NextApiResponse) {
  const uploadRequest = req.body as Upload

  console.log(`Upload request received: ${uploadRequest}`)

  res.status(200).end()
}
