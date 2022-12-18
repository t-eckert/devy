interface UploadRequest {
  id?: string
  url: URL
  status?:
    | "requested"
    | "queued"
    | "cloning"
    | "parsing"
    | "uploading"
    | "complete"
}

export default UploadRequest
