const url = "https://raw.githubusercontent.com/t-eckert/devy/main/CHANGELOG.md"

export const fetchChangelog = async (): Promise<string> => {
	const res = await fetch(url)
	return await res.text()
}
