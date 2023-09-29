const url = "https://raw.githubusercontent.com/t-eckert/devy/main/CHANGELOG.md"

export const fetchChangelog = async (): Promise<Option<string>> => {
	try {
		const res = await fetch(url)

		if (!res.ok) return null

		return await res.text()
	} catch (e) {
		console.log(e)
		return null
	}
}
