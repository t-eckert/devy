import config from "@/config"

const handleRequest = async <T>(
	method: string,
	path: string,
	revalidate: number,
	token?: string
): Promise<Option<T>> => {
	try {
		const response = await fetch(config.API + path, {
			method,
			headers: {
				"Content-Type": "application/json",
				...(token && { Authorization: `Bearer ${token}` }),
			},
			...(revalidate && { next: { revalidate } }),
		})

		if (!response.ok) return null

		return (await response.json()) as T
	} catch (error) {
		console.error(error)
		return null
	}
}

const get = async <T>(
	path: string,
	revalidate: number,
	token?: string
): Promise<Option<T>> => {
	return handleRequest("GET", path, revalidate, token)
}

const post = async <T>(
	path: string,
	revalidate: number,
	token?: string
): Promise<Option<T>> => {
	return handleRequest("POST", path, revalidate, token)
}

const api = {
	get,
	post,
}

export default api
