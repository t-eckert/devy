const url = "https://raw.githubusercontent.com/t-eckert/devy/main/CHANGELOG.md"

const get = {
	fromGitHub: async (): Promise<Option<string>> => {
		try {
			const res = await fetch(url)

			if (!res.ok) return null

			return await res.text()
		} catch (e) {
			console.log(e)
			return null
		}
	},
}

const changelogController = {
	get,
}

export default changelogController
