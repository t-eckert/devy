const mode = process.env.NODE_ENV
const local = process.env.USE_LOCAL

export interface Config {
	MODE: string
	HOST: string
	API: string
	LOCAL: boolean
}

const config: Config = {
	MODE: mode,
	HOST: mode === "production" ? "https://devy.page" : "http://localhost:3000",
	API:
		mode === "production"
			? "https://api.devy.page"
			: "http://localhost:8080",
	LOCAL: local === "true",
}

export default config
