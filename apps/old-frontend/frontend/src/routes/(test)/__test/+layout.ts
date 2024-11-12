import type { LayoutLoad } from "./$types"
import tests from "./tests"

export const load: LayoutLoad = async () => {
	return {
		props: {
			tests
		}
	}
}
