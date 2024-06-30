import type { PageServerLoad } from "./$types"

export const load: PageServerLoad = async () => {
	const response = await fetch(
		"https://raw.githubusercontent.com/t-eckert/devy/dev/changelog/README.md"
	)
	const body = await response.text()

	return {
		props: { body }
	}
}
