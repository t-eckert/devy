export default interface GitHubRepo {
	id: number
	name: string
	url: string
	html_url: string
	description: string
	fork: boolean
	language?: string
}
