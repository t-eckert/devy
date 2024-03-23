import config from "$lib/config"

const get = async <T,>(path: string): Promise<T> => {
	const response = await fetch(config.api + path, {
		method: "GET",
		headers: {
			"Content-Type": "application/json"
		}
	})

	return (await response.json()) as T
}

const set = async <T,>(path: string, body: T): Promise<T> => {
	const response = await fetch(config.api + path, {
		method: "POST",
		headers: {
			"Content-Type": "application/json"
		},
		body: JSON.stringify(body)
	})

	return (await response.json()) as T
}

const del = async <T,>(path: string): Promise<T> => {
	const response = await fetch(config.api + path, {
		method: "DELETE"
	})

	return (await response.json()) as T
}

const post = async <T,>(path: string, body: T): Promise<Response> => {
	return await fetch(config.api + path, {
		method: "POST",
		headers: {
			"Content-Type": "application/json"
		},
		body: JSON.stringify(body)
	})
}

const api = {
	get,
	set,
	delete: del,
	post
}

export default api
