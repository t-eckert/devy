export default interface GitHubRepo {
	id: number
	name: string
	html_url: string
	description: string
	fork: boolean
	language?: string
}
