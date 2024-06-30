import { error, NumericRange } from "@sveltejs/kit"
import config from "$lib/config"

const get = async <T>(path: string): Promise<T> => {
	const response = await fetch(config.api + path, {
		method: "GET",
		headers: {
			"Content-Type": "application/json"
		}
	})

	if (!response.ok && 400 <= response.status && response.status <= 599) {
		error(response.status as NumericRange<400, 599>)
	}

	const entity: T = await response.json()

	return entity
}

const getWithAuth = async <T>(path: string, token: string): Promise<T> => {
	const response = await fetch(config.api + path, {
		method: "GET",
		headers: {
			"Content-Type": "application/json",
			Authorization: `Bearer ${token}`
		}
	})

	if (!response.ok && 400 <= response.status && response.status <= 599) {
		error(response.status as NumericRange<400, 599>)
	}

	const entity: T = await response.json()

	return entity
}

const set = async <T>(path: string, body: T): Promise<T> => {
	const response = await fetch(config.api + path, {
		method: "POST",
		headers: {
			"Content-Type": "application/json"
		},
		body: JSON.stringify(body)
	})

	return (await response.json()) as T
}

const del = async <T>(path: string): Promise<T> => {
	const response = await fetch(config.api + path, {
		method: "DELETE"
	})

	return (await response.json()) as T
}

const post = async <T>(path: string, body: T): Promise<Response> => {
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
	getWithAuth,
	set,
	delete: del,
	post
}

export default api
