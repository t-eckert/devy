import config from "$lib/config"

const get = async <T,>(path: string): Promise<T> => {
	const response = await fetch(config.API + path, {
		method: "GET",
		headers: {
			"Content-Type": "application/json"
		}
	})

	return (await response.json()) as T
}

const post = async <T,>(path: string, body: T): Promise<T> => {
	const response = await fetch(config.API + path, {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
			Authorization: "Bearer 123"
		},
		body: JSON.stringify(body)
	})

	return (await response.json()) as T
}

const del = async <T,>(path: string): Promise<T> => {
	const response = await fetch(config.API + path, {
		method: "DELETE"
	})

	return (await response.json()) as T
}

const api = {
	get,
	post,
	delete: del
}

export default api
