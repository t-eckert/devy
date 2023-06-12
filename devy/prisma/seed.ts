const PrismaClient = require("@prisma/client").PrismaClient

const prisma = new PrismaClient()

async function seed() {
	const users = await prisma.user.createMany({
		data: [
			{
				id: 0,
				email: "dill@liminal.com",
				name: "Dill Pickles",
				username: "dillman",
			},
		],
		skipDuplicates: true,
	})
	console.log("Created Users, skipping duplicates")
	console.dir(users)

	const blogs = await prisma.blog.createMany({
		data: [
			{
				id: 0,
				authorId: 0,
				slug: "dillsblog",
				name: "Dill's Blog",
			},
		],
		skipDuplicates: true,
	})
	console.log("Created Blogs, skipping duplicates")
	console.dir(blogs)

	const posts = await prisma.post.createMany({
		data: [
			{
				id: 0,
				authorUsername: "dillman",
				blogSlug: "dillsblog",
				slug: "introduction-to-machine-learning",
				title: "Introduction to Machine Learning",
				markdown:
					"# Introduction to Machine Learning\n\nMachine learning is a powerful technique that enables computers to learn from data and make predictions or decisions. In this blog post, we will explore the basic concepts and algorithms used in machine learning.\n\n...",
			},
		],
		skipDuplicates: true,
	})
	console.log("Created Posts, skipping duplicates")
	console.dir(posts)
}

seed()
	.then(async () => {
		await prisma.$disconnect()
	})
	.catch(async (e) => {
		console.error(e)
		await prisma.$disconnect()
		process.exit(1)
	})
