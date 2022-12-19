import type { NextApiRequest, NextApiResponse } from "next"
import { UploadRequest } from "interfaces"

export default async function POST(req: NextApiRequest, res: NextApiResponse) {
  const uploadRequest = req.body as UploadRequest

  console.log(`Upload request received: ${uploadRequest}`)

  res.status(200).end()
}
