type Option<T> = T | null

type Controller<T> = {
	get: {}
	upsert: (t: T) => Promise<Option<T>>
}
