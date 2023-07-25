import db from "@/db"

export default interface Upload {
	id?: string
	user: string
}

export const isUpload = (obj: any): boolean => {
	return Object.hasOwn(obj, "user")
}

export const uploadCreator = {
	new: async (upload: Upload): Promise<Upload> => {
		console.log("Creating upload!")

		upload.id = "bd6455cf-8087-4bb8-94fc-da926f79e0e0"
		console.dir(upload)

		console.log("Upload created!")

		return upload
	},
}

export const uploadGetter = {
	byId: async (id: string): Promise<Upload | null> => {
		const upload = await db.upload.findUnique({
			where: { id },
		})

		console.log(upload)

		return upload
	},
}