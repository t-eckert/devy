import config from "@/config"

const get = async <T>(path: string, revalidate: number): Promise<T> => {
	const response = await fetch(config.API + path, {
		method: "GET",
		headers: {
			"Content-Type": "application/json",
		},
		...(revalidate && { next: { revalidate } }),
	})

	return (await response.json()) as T
}

const post = async <T>(path: string, body: T): Promise<T> => {
	const response = await fetch(config.API + path, {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
			Authorization: "Bearer 123",
		},
		body: JSON.stringify(body),
	})

	return (await response.json()) as T
}

const submit = async <T>(path: string, body: any): Promise<T> => {
	const response = await fetch(config.API + path, {
		method: "POST",
		headers: {
			"Content-Type": "application/x-www-form-urlencoded",
			Authorization: "Bearer 123",
		},
		body: Object.keys(body)
			.map(
				(key) =>
					encodeURIComponent(key) +
					"=" +
					encodeURIComponent(body[key])
			)
			.join("&"),
	})

	return (await response.json()) as T
}

const del = async <T>(path: string): Promise<T> => {
	const response = await fetch(config.API + path, {
		method: "DELETE",
	})

	return (await response.json()) as T
}

const api = {
	get,
	post,
	submit,
	delete: del,
}

export default api
