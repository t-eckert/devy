export interface Config {
	MODE: "development" | "production" | "test"
	API: string
}

const mode = process.env.NODE_ENV

const config: Config = {
	MODE: mode,
	API: (() => {
		switch (mode) {
			case "development":
				return "http://localhost:8000"
			case "production":
				return "https://api.devy.page"
			case "test":
				return "http://localhost:8000"
			default:
				return "http://localhost:8000"
		}
	})(),
}

export default config
