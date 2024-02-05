export interface Config {
	version: string
	host: string
	api: string
	mode: "development" | "production"
}

const config: Config = {
	version: "0.4.0",
	host: "http://localhost:5173",
	api: "http://localhost:8000",
	mode: "development"
}

export default config
