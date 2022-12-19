import type { NextApiRequest, NextApiResponse } from "next"
import upload from "uploader/uploader"
import { UploadRequest } from "interfaces"

export default async function POST(req: NextApiRequest, res: NextApiResponse) {
  const uploadRequest = req.body as UploadRequest

  const response = await upload(uploadRequest)
  console.table(response)

  res.status(200).end()
}
