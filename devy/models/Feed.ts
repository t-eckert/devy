import db from "@/db"
import { Tag } from "@prisma/client"
import Post from "./Post"
import type Translator from "@/interfaces/Translator"

export default interface Feed {
	id: number
	slug: string
	name: string
	publishOrder: string
	filterTags: Tag[]
	lastCachedAt: Date
	posts: Post[]
}

export const feedTranslator: Translator<any, any> = {
	toModel: (prisma) => ({}),
	toPrisma: (model) => ({}),
}

export const feedCreator = {}

export const feedGetter = {
	default: async (): Promise<Feed[]> => {
		return [
			{
				id: 1,
				slug: "popular",
				name: "Popular",
				publishOrder: "desc",
				filterTags: [],
				lastCachedAt: new Date(),
				posts: posts.sort((a, b) => (a.likes > b.likes ? -1 : 1)),
			},
			{
				id: 2,
				slug: "new",
				name: "New",
				publishOrder: "desc",
				filterTags: [],
				lastCachedAt: new Date(),
				posts: posts.sort((a, b) =>
					a.published > b.published ? -1 : 1
				),
			},
		]
	},
}

const posts: Post[] = [
	{
		slug: "introduction-to-machine-learning",
		blog: "ml-blog",
		author: {
			username: "johndoe",
			displayName: "John Doe",
		},
		title: "Introduction to Machine Learning",
		published: "2022-03-15",
		updated: "2022-03-15",
		tags: ["machine learning", "data science"],
		likes: 42,
		markdown:
			"# Introduction to Machine Learning\n\nMachine learning is a powerful technique that enables computers to learn from data and make predictions or decisions. In this blog post, we will explore the basic concepts and algorithms used in machine learning.\n\n...",
	},
	{
		slug: "building-scalable-web-applications",
		blog: "web-dev-blog",
		author: {
			username: "janedoe",
			displayName: "Jane Doe",
		},
		title: "Building Scalable Web Applications",
		published: "2022-07-02",
		updated: "2022-07-02",
		tags: ["web development", "scalability"],
		likes: 27,
		markdown:
			"# Building Scalable Web Applications\n\nIn today's digital world, building web applications that can handle a large number of users and scale effectively is crucial. In this blog post, we will discuss various strategies and best practices for building scalable web applications.\n\n...",
	},
	{
		slug: "data-visualization-with-d3",
		blog: "data",
		author: {
			username: "johndoe",
			displayName: "John Doe",
		},
		title: "Data Visualization with D3",
		published: "2022-09-21",
		updated: "2022-09-21",
		tags: ["data visualization", "frontend"],
		likes: 63,
		markdown:
			"# Data Visualization with D3\n\nData visualization is a powerful way to represent and communicate complex data effectively. In this blog post, we will explore how to use D3.js, a popular JavaScript library, to create interactive and visually appealing data visualizations.\n\n...",
	},
	{
		slug: "getting-started-with-python",
		blog: "python-blog",
		author: {
			username: "johndoe",
			displayName: "John Doe",
		},
		title: "Getting Started with Python",
		published: "2022-10-10",
		updated: "2022-10-10",
		tags: ["python", "programming"],
		likes: 88,
		markdown:
			"# Getting Started with Python\n\nPython is a versatile programming language that is widely used for various purposes, including web development, data analysis, and artificial intelligence. In this blog post, we will cover the basics of Python programming and get you started on your Python journey.\n\n...",
	},
	{
		slug: "secure-coding-practices",
		blog: "security",
		author: {
			username: "allela",
			displayName: "Allela Kilonzo",
		},
		title: "Secure Coding Practices",
		published: "2022-11-28",
		updated: "2022-11-28",
		tags: ["security", "programming"],
		likes: 54,
		markdown:
			"# Secure Coding Practices\n\nWriting secure code is of utmost importance in today's digital landscape. In this blog post, we will discuss essential secure coding practices that can help protect your applications from common vulnerabilities and attacks.\n\n...",
	},
	{
		slug: "understanding-algorithms",
		blog: "algorithms",
		author: {
			username: "harry",
			displayName: "Harry Larson",
		},
		title: "Understanding Algorithms",
		published: "2023-01-05",
		updated: "2023-01-05",
		tags: ["algorithms", "computer science"],
		likes: 39,
		markdown:
			"# Understanding Algorithms\n\nAlgorithms form the foundation of computer science and are essential for solving complex problems efficiently. In this blog post, we will dive into various algorithms, their analysis, and their practical applications.\n\n...",
	},
	{
		slug: "deploying-containerized-applications",
		blog: "devops",
		author: {
			username: "peggy",
			displayName: "Peggy Brown",
		},
		title: "Deploying Containerized Applications",
		published: "2023-02-19",
		tags: ["containerization", "devops"],
		likes: 72,
		markdown:
			"# Deploying Containerized Applications\n\nContainerization has revolutionized the way software is developed and deployed. In this blog post, we will explore the benefits of containerization and learn how to deploy containerized applications using popular container orchestration platforms.\n\n...",
	},
	{
		slug: "introduction-to-react",
		blog: "frontend",
		author: {
			username: "johndoe",
			displayName: "John Doe",
		},
		title: "Introduction to React",
		published: "2023-04-09",
		tags: ["react", "frontend"],
		likes: 65,
		markdown:
			"# Introduction to React\n\nReact is a popular JavaScript library for building user interfaces. In this blog post, we will introduce the key concepts of React and guide you through the process of creating your first React application.\n\n...",
	},
	{
		slug: "data-structures-in-java",
		blog: "java",
		author: {
			username: "johndoe",
			displayName: "John Doe",
		},
		title: "Data Structures in Java",
		published: "2023-05-22",
		updated: "2023-05-22",
		tags: ["java", "data structures"],
		likes: 49,
		markdown:
			"# Data Structures in Java\n\nData structures are essential for organizing and manipulating data efficiently. In this blog post, we will explore various data structures implemented in Java and discuss their characteristics, use cases, and performance implications.\n\n...",
	},
]
