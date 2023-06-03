export default interface Tab {
	slug: string
	title: string
	count?: number
}

export const getTabs = (): Tab[] => {
	return tabs
}

const tabs: Tab[] = [
	{
		slug: "popular",
		title: "Popular",
	},
	{
		slug: "new",
		title: "New",
		count: 10,
	},
]
