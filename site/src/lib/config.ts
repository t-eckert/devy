export interface Config {
	VERSION: string
	HOST: string
	API: string
}

const config: Config = {
	VERSION: "0.4.0",
	HOST: "http://localhost:3000",
	API: "http://localhost:8000"
}

export default config
