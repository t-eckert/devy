import Upload, { uploadCreator } from "@/models/Upload"

export default class Uploader {
	upload: Upload

	constructor(upload: Upload) {
		this.upload = upload
	}

	async start(): Promise<string> {
		const upload = await uploadCreator.new(this.upload)

		if (!upload.id) {
			return Promise.reject(
				"Upload was not assigned an id. This is likely an error that occurred when communicating with the database."
			)
		}

		return upload.id
	}
}
