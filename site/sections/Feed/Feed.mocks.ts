import { Props } from "./Feed"

const popularFeed: Props = {
	postsMetadata: [
		{
			title:
				"They Didn't Tell Me I Had to Eat the Two Pizzas: Stories of an IC Turned Manager",
			slug: "they-didnt-tell-me-i-had-to-eat-the-two-pizzas",
			author: {
				id: "95781875-ba3f-417c-9b4b-dec6b6aea354",
				name: "Demari Williams",
				username: "d-williams",
			},
			updatedAt: "2023-01-04",
			tags: ["management", "humor"],
			likes: 300,
		},
		{
			title: "10 Reasons Why Blockchain is the Future of Finance",
			slug: "10-reasons-why-blockchain-is-the-future-of-finance",
			author: {
				id: "8f24f66e-7f32-4d21-9fca-61c2e7bce651",
				name: "Lena Nguyen",
				username: "lnguyen",
			},
			updatedAt: "2022-12-30",
			tags: ["blockchain", "finance"],
			likes: 187,
		},
		{
			title:
				"The Top 5 Machine Learning Libraries Every Data Scientist Should Know",
			slug: "the-top-5-machine-learning-libraries-every-data-scientist-should-know",
			author: {
				id: "a5fb5b43-6e7c-4059-b973-e8f56b1e2d41",
				name: "Mason Kim",
				username: "mkim",
			},
			updatedAt: "2022-12-27",
			tags: ["machine learning", "data science"],
			likes: 432,
		},
		{
			title: "How to Build a Scalable Serverless Architecture on AWS",
			slug: "how-to-build-a-scalable-serverless-architecture-on-aws",
			author: {
				id: "c6e8f3f3-3b0c-4f9d-8a7c-bbe4fad6e497",
				name: "Ashley Rodriguez",
				username: "arodriguez",
			},
			updatedAt: "2022-12-23",
			tags: ["serverless", "AWS"],
			likes: 271,
		},
		{
			title: "The Future of Virtual Reality: Predictions for 2022 and Beyond",
			slug: "the-future-of-virtual-reality-predictions-for-2022-and-beyond",
			author: {
				id: "8f0b2eea-9f7b-43a9-8a64-1b78a4b4e4c0",
				name: "Samantha Kim",
				username: "skim",
			},
			updatedAt: "2022-12-20",
			tags: ["virtual reality", "predictions"],
			likes: 389,
		},
		{
			title:
				"The Pros and Cons of Flutter vs. React Native for Mobile App Development",
			slug: "the-pros-and-cons-of-flutter-vs-react-native-for-mobile-app-development",
			author: {
				id: "eebdf74a-d07d-4d65-a7aa-6e5f6cd75c1d",
				name: "Emily Lee",
				username: "elee",
			},
			updatedAt: "2022-12-17",
			tags: ["mobile app development", "Flutter", "React Native"],
			likes: 309,
		},
		{
			title: "3 Ways Artificial Intelligence is Changing Healthcare",
			slug: "3-ways-artificial-intelligence-is-changing-healthcare",
			author: {
				id: "9f2c9e9b-7b34-4e8a-a497-f1b3e6b3c1e3",
				name: "David Kim",
				username: "dkim",
			},
			updatedAt: "2022-12-13",
			tags: ["artificial intelligence", "healthcare"],
			likes: 256,
		},
		{
			title:
				"The Benefits and Challenges of Using Microservices in Your Architecture",
			slug: "the-benefits-and-challenges-of-using-microservices-in-your-architecture",
			author: {
				id: "a7b5edc4-a9f9-4eee-b7b7-da8e3a3a80e3",
				name: "Sophia Williams",
				username: "swilliams",
			},
			updatedAt: "2022-12-09",
			tags: ["microservices", "architecture"],
			likes: 187,
		},
		{
			title:
				"An Introduction to Quantum Computing: What It Is and How It Works",
			slug: "an-introduction-to-quantum-computing-what-it-is-and-how-it-works",
			author: {
				id: "3c13f937-6e8a-4c7a-bdfd-b7eb96384699",
				name: "Daniel Kim",
				username: "dkim",
			},
			updatedAt: "2022-12-06",
			tags: ["quantum computing"],
			likes: 341,
		},
	],
	feeds: [
		{ id: "37e24b45-e5d2-456e-94ef-11f2c64ef17f", name: "Popular" },
		{ id: "aee0f9e9-a028-4c8b-a453-dbdbcf309fc0", name: "New" },
	],
	feedId: "37e24b45-e5d2-456e-94ef-11f2c64ef17f",
	setFeedId: (_: string) => { },
	pageNumber: 1,
	setPageNumber: (_: number) => { },
}

const mocks = {
	popularFeed,
}

export default mocks
