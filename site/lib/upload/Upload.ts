interface Upload {
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

export default Upload
