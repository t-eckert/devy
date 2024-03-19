export interface Config {
	host: string
	api: string
}

const localConfig: Config = {
	host: "http://localhost:3000",
	api: "http://localhost:8000"
}

const config: Config = {
	host: process.env.HOST ?? localConfig.host,
	api: process.env.API ?? localConfig.api
}

export default config
