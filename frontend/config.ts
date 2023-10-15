const mode = process.env.NODE_ENV

export interface Config {
	MODE: string
	HOST: string
	API: string
}

const config: Config = {
	MODE: mode,
	HOST: mode === "production" ? "https://devy.page" : "http://localhost:3000",
	API:
		mode === "production"
			? "https://api.devy.page/v1"
			: "http://localhost:8000/v1",
}

export default config
