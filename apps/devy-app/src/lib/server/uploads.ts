import { UPLOADER_URL } from "$env/static/private"

export interface UploadRequest {
  blogId: string
}

export interface UploadResponse {
  uploadId: string
}

export interface Upload {
  id: string
}

// Sends an upload request to the uploader.
export async function send(uploadRequest: UploadRequest): Promise<UploadResponse> {
  const resp = await fetch(UPLOADER_URL, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(uploadRequest)
  })

  if (!resp.ok) {
    throw new Error("Failed to send upload request.")
  }

  return await resp.json()
}

// Fetches an upload from the uploader.
export async function get(uploadId: string): Promise<Upload> {
  const resp = await fetch(`${UPLOADER_URL}/${uploadId}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json"
    },
  })

  if (!resp.ok) {
    throw new Error("Failed to send upload request.")
  }

  return await resp.json()
}
