import { exec } from "node:child_process"

import { Post, UploadRequest } from "interfaces"

export default async function upload(
	uploadRequest: UploadRequest
): Promise<UploadRequest> {
	uploadRequest.id = "51e9a8c1-f5a4-42a7-9a80-f0b5dec6ff59"
	uploadRequest.status = "queued"

	const path = clone(uploadRequest.url, uploadRequest.id)
	const posts = parse(path)

	return uploadRequest
}

// clone uses git to clone the given repository.
// If the clone is successful, the path to the repository is returned.
const clone = (repo: URL, id: string): string => {
	console.table(repo)

	const path = `./public/repos/${id}`
	exec(`git clone ${repo} ${path}`, (e) => {
		if (e) {
			throw e
		}
	})

	return path
}

const parse = (path: string): Post[] => {
	return []
}

const cleanup = (path: string) => {
	exec(`rm -rf ${path}`, (e) => {
		if (e) {
			throw e
		}
	})
}
