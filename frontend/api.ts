import config from "@/config"

const handleRequest = async <T>(
	path: string,
	options?: RequestInit
): Promise<Option<T>> => {
	try {
		const response = await fetch(config.API + path, options)

		if (!response.ok) return null

		return (await response.json()) as T
	} catch (error) {
		console.error(error)
		return null
	}
}

const get = async <T>(
	path: string,
	options?: RequestInit
): Promise<Option<T>> => {
	return handleRequest(path, { method: "GET", ...options })
}

const post = async <T>(
	path: string,
	options?: RequestInit
): Promise<Option<T>> => {
	return handleRequest(path, { method: "POST", ...options })
}

const put = async <T>(
	path: string,
	options?: RequestInit
): Promise<Option<T>> => {
	return handleRequest(path, { method: "PUT", ...options })
}

const api = {
	get,
	post,
	put,
}

export default api
