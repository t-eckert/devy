function panic(message: string): never {
	throw new Error(message)
}

export interface Config {
	HOST: string
	API: string
}

const config: Config = {
	HOST: process.env.HOST || panic("HOST not set"),
	API: process.env.API || panic("API not set"),
}

export default config
