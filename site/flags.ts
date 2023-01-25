import client from "client"

export interface Flag {
	key: string
	value: string
	description: string
}

export const getFlags = async (): Promise<Map<string, string>> => {
	const { data, error } = await client
		.from("flag")
		.select("key, value, description")
	if (error) {
		throw error
	}

	console.log(data)
	const flags = new Map<string, string>()
	data.forEach((flag: Flag) => {
		flags.set(flag.key, flag.value)
	})
	console.log(flags)
	return flags
}
