function panic(message: string): never {
	throw new Error(message)
}

export interface Config {
	HOST: string
	API: string
}

const config: Config = {
	HOST: process.env.HOST ?? "http://localhost:3000",
	API: process.env.API ?? "http://localhost:8000",
}

export default config
