type Option<T> = T | null

type Controller<T> = {
	get: any
	upsert: (t: T) => Promise<Option<T>>
}
