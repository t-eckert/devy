const mode = process.env.NODE_ENV

export interface Config {
	MODE: string
	HOST: string
	AUTH: {
		GITHUB: {
			ID: string
			SECRET: string
		}
	}
}

const config: Config = {
	MODE: mode,
	HOST: mode === "production" ? "https://devy.page" : "http://localhost:3000",
	AUTH: {
		GITHUB: {
			ID: process.env.GITHUB_ID || "",
			SECRET: process.env.GITHUB_SECRET || "",
		},
	},
}

export default config
