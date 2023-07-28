const mode = process.env.NODE_ENV

export interface Config {
	MODE: string
	HOST: string
}

const config: Config = {
	MODE: mode,
	HOST: mode === "production" ? "https://devy.page" : "http://localhost:3000",
}

export default config
